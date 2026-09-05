use modus_sdk::auth_token;
use modus_sdk::bus_emit;
use modus_sdk::log::{self, Level};
use modus_sdk::net_ws;
use modus_sdk::next_backoff_ms;
use modus_sdk::wait::{self, Ready};
use modus_sdk::wait_backoff;
use modus_sdk::{Guest, HostError, BACKOFF_START_MS};

const WS_URL: &str = "wss://example.com/";

struct Plugin;

enum Outcome {
    Stopped,
    Retry,
}

impl Guest for Plugin {
    fn init() {
        log::log(Level::Info, "init");
    }

    fn run() {
        let accounts = auth_token::list_accounts();
        if accounts.is_empty() {
            log::log(Level::Info, "no account");
            loop {
                if matches!(wait::wait(), Ready::Stop) {
                    return;
                }
            }
        }
        run_account(&accounts[0]);
    }

    fn shutdown() {}
}

fn run_account(account_id: &str) {
    let mut backoff = BACKOFF_START_MS;
    loop {
        match run_session(account_id) {
            Outcome::Stopped => {
                log::log(Level::Info, "stopped");
                return;
            }
            Outcome::Retry => {
                if wait_backoff(backoff) {
                    log::log(Level::Info, "stopped");
                    return;
                }
                backoff = next_backoff_ms(backoff);
            }
        }
    }
}

fn run_session(account_id: &str) -> Outcome {
    if let Err(err) = auth_token::token(account_id) {
        return fail(&err);
    }
    let handle = match net_ws::connect(WS_URL) {
        Ok(handle) => handle,
        Err(err) => return fail(&err),
    };
    loop {
        match wait::wait() {
            Ready::Stop => {
                let _ = net_ws::close(handle);
                return Outcome::Stopped;
            }
            Ready::WsClosed(_) => {
                let _ = net_ws::close(handle);
                return fail("ws closed");
            }
            Ready::WsText(frame) => {
                let payload = modus_sdk::text_message("dev", "dev", frame.text, None, None);
                if let Err(err) = bus_emit::emit("example", &payload, None) {
                    if HostError::classify(&err).is_stop() {
                        let _ = net_ws::close(handle);
                        return Outcome::Stopped;
                    }
                    log::log(Level::Warn, &err);
                }
            }
            Ready::Act(req) => {
                modus_sdk::chat_complete::complete(&req.id, Err("no connection"));
            }
            Ready::Timer | Ready::Bus(_) | Ready::Settings | Ready::Resume | Ready::Ui(_) | Ready::MediaEnded(_)
                | Ready::AlertPlay(_)
                | Ready::AlertStop(_) => {}
        }
    }
}

fn fail(err: &str) -> Outcome {
    let classified = HostError::classify(err);
    if classified == HostError::Stopped {
        return Outcome::Stopped;
    }
    log::log(Level::Warn, err);
    if classified.is_stop() {
        Outcome::Stopped
    } else {
        Outcome::Retry
    }
}

modus_sdk::export!(Plugin);
