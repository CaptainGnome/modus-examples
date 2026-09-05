use modus_sdk::bus_emit;
use modus_sdk::donation;
use modus_sdk::log::{self, Level};
use modus_sdk::text_fragment;
use modus_sdk::text_message;
use modus_sdk::wait::{self, Ready};
use modus_sdk::Guest;

struct Plugin;

impl Guest for Plugin {
    fn init() {
        log::log(Level::Info, "emitter init");
        // Educational bus source: same idea as `modus dev` built-in fixture.
        let message = text_message("emitter", "emitter", "fixture hello", None, None);
        if let Err(err) = bus_emit::emit("dev", &message, None) {
            log::log(Level::Error, &err);
        }
        let donation = donation(
            "emitter",
            "emitter",
            5.0,
            "USD",
            vec![text_fragment("thanks for the stream!")],
        );
        if let Err(err) = bus_emit::emit("dev", &donation, None) {
            log::log(Level::Error, &err);
        }
        wait::subscribe();
    }

    fn run() {
        loop {
            match wait::wait() {
                Ready::Stop => return,
                Ready::Act(req) => {
                    modus_sdk::chat_complete::complete(&req.id, Err("нет соединения"));
                }
                Ready::Bus(_)
                | Ready::WsText(_)
                | Ready::WsClosed(_)
                | Ready::Timer
                | Ready::Settings
                | Ready::Resume
                | Ready::Ui(_)
                | Ready::MediaEnded(_)
                | Ready::AlertPlay(_)
                | Ready::AlertStop(_) => {}
            }
        }
    }

    fn shutdown() {}
}

modus_sdk::export!(Plugin);
