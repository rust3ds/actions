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

    fn cleanup(self, test_result: std::io::Result<bool>) {
        // GDB actually has the opportunity to inspect the exit code,
        // unlike other runners, so let's follow the default behavior of the
        // stdlib test runner.
        match test_result {
            Ok(success) => {
                if success {
                    std::process::exit(0);
                } else {
                    std::process::exit(101);
                }
            }
            Err(err) => {
                eprintln!("Error: {err}");
                std::process::exit(101);
            }
        }
    }
}
