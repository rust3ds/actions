use ctru::error::ResultCode;

use super::TestRunner;
use crate::TestResult;

#[derive(Default)]
pub struct GdbRunner;

impl Drop for GdbRunner {
    fn drop(&mut self) {
        unsafe { ctru_sys::gdbHioDevExit() }
    }
}

impl TestRunner for GdbRunner {
    type Context<'this> = ();

    fn setup(&mut self) -> Self::Context<'_> {
        // TODO: `ctru` expose safe API to do this and call that instead
        || -> ctru::Result<()> {
            unsafe {
                ResultCode(ctru_sys::gdbHioDevInit())?;
                // TODO: should we actually redirect stdin or nah?
                ResultCode(ctru_sys::gdbHioDevRedirectStdStreams(true, true, true))?;
            }
            Ok(())
        }()
        .expect("failed to redirect I/O streams to GDB");
    }

    fn cleanup<T: TestResult>(self, test_result: T) -> T {
        // GDB actually has the opportunity to inspect the exit code,
        // unlike other runners, so let's follow the default behavior of the
        // stdlib test runner.
        std::process::exit(if test_result.succeeded() { 0 } else { 101 })
    }
}
