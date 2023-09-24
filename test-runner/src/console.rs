use ctru::prelude::*;
use ctru::services::gfx::{Flush, Swap};

use super::TestRunner;

pub struct ConsoleRunner {
    gfx: Gfx,
    hid: Hid,
    apt: Apt,
}

impl Default for ConsoleRunner {
    fn default() -> Self {
        let gfx = Gfx::new().unwrap();
        let hid = Hid::new().unwrap();
        let apt = Apt::new().unwrap();

        gfx.top_screen.borrow_mut().set_wide_mode(true);

        Self { gfx, hid, apt }
    }
}

impl TestRunner for ConsoleRunner {
    type Context<'this> = Console<'this>;

    fn setup(&mut self) -> Self::Context<'_> {
        Console::new(self.gfx.top_screen.borrow_mut())
    }

    fn cleanup(mut self, _test_result: std::io::Result<bool>) {
        // We don't actually care about the test result, either way we'll stop
        // and show the results to the user

        // Wait to make sure the user can actually see the results before we exit
        println!("Press START to exit.");

        while self.apt.main_loop() {
            let mut screen = self.gfx.top_screen.borrow_mut();
            screen.flush_buffers();
            screen.swap_buffers();

            self.gfx.wait_for_vblank();

            self.hid.scan_input();
            if self.hid.keys_down().contains(KeyPad::START) {
                break;
            }
        }
    }
}
