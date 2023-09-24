//! Custom test runner for building/running tests on the 3DS.
//!
//! This library can be used with
//! [`custom_test_frameworks`](https://doc.rust-lang.org/unstable-book/language-features/custom-test-frameworks.html)
//! to enable normal Rust testing workflows for 3DS homebrew.

#![feature(test)]
#![feature(custom_test_frameworks)]
#![test_runner(run_gdb)]

extern crate test;

mod console;
mod gdb;
mod socket;

use console::ConsoleRunner;
use gdb::GdbRunner;
use socket::SocketRunner;

use test::{ColorConfig, OutputFormat, TestDescAndFn, TestFn, TestOpts};

/// Show test output in GDB, using the [File I/O Protocol] (called HIO in some 3DS
/// homebrew resources). Both stdout and stderr will be printed to the GDB console.
///
/// [File I/O Protocol]: https://sourceware.org/gdb/onlinedocs/gdb/File_002dI_002fO-Overview.html#File_002dI_002fO-Overview
pub fn run_gdb(tests: &[&TestDescAndFn]) {
    run::<GdbRunner>(tests)
}

/// Run tests using the `ctru` [`Console`] (print results to the 3DS screen).
/// This is mostly useful for running tests manually, especially on real hardware.
///
/// [`Console`]: ctru::console::Console
pub fn run_console(tests: &[&TestDescAndFn]) {
    run::<ConsoleRunner>(tests)
}

/// Show test output via a network socket to `3dslink`. This runner is only useful
/// on real hardware, since `3dslink` doesn't work with emulators.
///
/// See [`Soc::redirect_to_3dslink`] for more details.
///
/// [`Soc::redirect_to_3dslink`]: ctru::services::soc::Soc::redirect_to_3dslink
pub fn run_socket(tests: &[&TestDescAndFn]) {
    run::<SocketRunner>(tests)
}

fn run<Runner: TestRunner>(tests: &[&TestDescAndFn]) {
    std::env::set_var("RUST_BACKTRACE", "1");

    let mut runner = Runner::default();
    let ctx = runner.setup();

    let opts = TestOpts {
        force_run_in_process: true,
        run_tests: true,
        // TODO: color doesn't work because of TERM/TERMINFO.
        // With RomFS we might be able to fake this out nicely...
        color: ColorConfig::AlwaysColor,
        format: OutputFormat::Pretty,
        test_threads: Some(1),
        // Hopefully this interface is more stable vs specifying individual options,
        // and parsing the empty list of args should always work, I think.
        // TODO Ideally we could pass actual std::env::args() here too
        ..test::test::parse_opts(&[]).unwrap().unwrap()
    };

    let tests = tests.iter().map(|t| make_owned_test(t)).collect();
    let result = test::run_tests_console(&opts, tests);

    drop(ctx);

    runner.cleanup(result);
}

/// Adapted from [`test::make_owned_test`].
/// Clones static values for putting into a dynamic vector, which `test_main()`
/// needs to hand out ownership of tests to parallel test runners.
///
/// This will panic when fed any dynamic tests, because they cannot be cloned.
fn make_owned_test(test: &TestDescAndFn) -> TestDescAndFn {
    let testfn = match test.testfn {
        TestFn::StaticTestFn(f) => TestFn::StaticTestFn(f),
        TestFn::StaticBenchFn(f) => TestFn::StaticBenchFn(f),
        _ => panic!("non-static tests passed to test::test_main_static"),
    };

    TestDescAndFn {
        testfn,
        desc: test.desc.clone(),
    }
}

/// A helper trait to make the behavior of test runners consistent.
trait TestRunner: Sized + Default {
    /// Any context the test runner needs to remain alive for the duration of
    /// the test. This can be used for things that need to borrow the test runner
    /// itself.
    // TODO: with associated type defaults this could be `= ();`
    type Context<'this>
    where
        Self: 'this;

    /// Create the [`Context`](Self::Context), if any.
    fn setup(&mut self) -> Self::Context<'_>;

    /// Handle the results of the test and perform any necessary cleanup.
    /// The [`Context`](Self::Context) will be dropped just before this is called.
    fn cleanup(self, test_result: std::io::Result<bool>);
}

/// This module has stubs needed to link the test library, but they do nothing
/// because we don't actually need them for the runner to work.
mod link_fix {
    #[no_mangle]
    extern "C" fn execvp(
        _argc: *const libc::c_char,
        _argv: *mut *const libc::c_char,
    ) -> libc::c_int {
        -1
    }

    #[no_mangle]
    extern "C" fn pipe(_fildes: *mut libc::c_int) -> libc::c_int {
        -1
    }

    #[no_mangle]
    extern "C" fn sigemptyset(_arg1: *mut libc::sigset_t) -> ::libc::c_int {
        -1
    }
}

/// Verify that doctests work as expected
/// ```
/// assert_eq!(2 + 2, 4);
/// ```
///
/// ```should_panic
/// assert_eq!(2 + 2, 5);
/// ```
#[cfg(doctest)]
struct Dummy;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    #[should_panic]
    fn it_fails() {
        assert_eq!(2 + 2, 5);
    }
}
