use ctru::prelude::*;

use super::TestRunner;

pub struct SocketRunner {
    soc: Soc,
}

impl Default for SocketRunner {
    fn default() -> Self {
        Self {
            soc: Soc::new().expect("failed to initialize network service"),
        }
    }
}

impl TestRunner for SocketRunner {
    type Context<'this> = ();

    fn setup(&mut self) -> Self::Context<'_> {
        self.soc
            .redirect_to_3dslink(true, true)
            .expect("failed to redirect to socket");
    }
}
