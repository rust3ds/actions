use std::process::Termination;

use ctru::error::ResultCode;

use super::TestRunner;

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

    fn cleanup<T: Termination>(self, test_result: T) -> T {
        // GDB actually has the opportunity to inspect the exit code,
        // unlike other runners, so let's follow the default behavior of the
        // stdlib test runner.
        test_result.report().exit_process()
    }
}
