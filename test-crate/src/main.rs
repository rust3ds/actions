use std::time::Duration;

use ctru::console::Console;
use ctru::gfx::Gfx;
use ctru::services::apt::Apt;
use ctru::services::hid::{Hid, KeyPad};

fn main() {
    ctru::init();
    let gfx = Gfx::init().expect("Couldn't obtain GFX controller");
    let hid = Hid::init().expect("Couldn't obtain HID controller");
    let apt = Apt::init().expect("Couldn't obtain APT controller");
    // let _console = Console::init(gfx.top_screen.borrow_mut());

    let _ = unsafe { ctru_sys::consoleDebugInit(ctru_sys::debugDevice_SVC) };

    std::env::set_var("RUST_BACKTRACE", "full");

    let res = unsafe { ctru_sys::gdbHioDevInit() };
    if res != 0 {
        eprintln!("failed to init gdbHIO: {res}");
    } else {
        eprintln!("init gdb hio");
    }

    // let res = unsafe { ctru_sys::gdbHioDevRedirectStdStreams(false, true, true) };
    // if res != 0 {
    //     eprintln!("failed to redirect gdbHIO: {res}");
    // } else {
    //     eprintln!("redirected gdb hio");
    // }

    println!("hey stdout");
    eprintln!("hey stderr");

    // Main loop
    while apt.main_loop() {
        //Scan all the inputs. This should be done once for each frame
        hid.scan_input();

        if hid.keys_down().contains(KeyPad::KEY_START) {
            break;
        }
        // Flush and swap framebuffers
        gfx.flush_buffers();
        gfx.swap_buffers();

        //Wait for VBlank
        gfx.wait_for_vblank();
    }

    unsafe { ctru_sys::gdbHioDevExit() };
}
