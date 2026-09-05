use modus_sdk::log::{self, Level};
use modus_sdk::ui_slot;
use modus_sdk::wait::{self, Ready};
use modus_sdk::Guest;

struct Plugin;

fn post_n(n: u32) {
    let body = format!("{{\"n\":{n}}}");
    if let Err(err) = ui_slot::post(body.as_bytes()) {
        log::log(Level::Warn, &err);
    }
}

impl Guest for Plugin {
    fn init() {
        log::log(Level::Info, "init");
        post_n(0);
    }

    fn run() {
        let mut n = 0u32;
        loop {
            match wait::wait() {
                Ready::Stop => return,
                Ready::Ui(_) => {
                    n = n.saturating_add(1);
                    post_n(n);
                }
                Ready::Bus(_)
                | Ready::WsText(_)
                | Ready::WsClosed(_)
                | Ready::Timer
                | Ready::Act(_)
                | Ready::Settings
                | Ready::Resume
                | Ready::MediaEnded(_)
                | Ready::AlertPlay(_)
                | Ready::AlertStop(_) => {}
            }
        }
    }

    fn shutdown() {}
}

modus_sdk::export!(Plugin);
