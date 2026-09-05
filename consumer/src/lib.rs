use modus_sdk::log::{self, Level};
use modus_sdk::types::{Fragment, Payload};
use modus_sdk::wait::{self, Event, Ready};
use modus_sdk::Guest;

struct Plugin;

impl Guest for Plugin {
    fn init() {
        log::log(Level::Info, "init");
        wait::subscribe();
    }

    fn run() {
        loop {
            match wait::wait() {
                Ready::Stop => return,
                Ready::Bus(event) => log_bus(&event),
                Ready::WsText(_)
                | Ready::WsClosed(_)
                | Ready::Timer
                | Ready::Act(_)
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

fn log_bus(event: &Event) {
    log::log(
        Level::Info,
        &format!(
            "bus {} {}:{} hide={} skip={} hi={} mask={} {}",
            payload_kind(&event.payload),
            event.source.plugin_id,
            event.source.channel,
            event.flags.hide_chat,
            event.flags.skip_alert,
            event.flags.highlight,
            event.flags.mask.as_deref().unwrap_or("-"),
            payload_text(&event.payload)
        ),
    );
}

fn payload_kind(payload: &Payload) -> &'static str {
    match payload {
        Payload::Message(_) => "message",
        Payload::Donation(_) => "donation",
        Payload::Sub(_) => "sub",
        Payload::Follow(_) => "follow",
        Payload::Raid(_) => "raid",
        Payload::ViewerCount(_) => "viewer_count",
        Payload::Reward(_) => "reward",
        Payload::Moderation(_) => "moderation",
        Payload::System(_) => "system",
        Payload::Custom(_) => "custom",
    }
}

fn payload_text(payload: &Payload) -> String {
    match payload {
        Payload::Message(msg) => fragments_text(&msg.fragments),
        Payload::Donation(don) => {
            let text = fragments_text(&don.fragments);
            if text.is_empty() {
                format!("{} {}", don.money.amount, don.money.currency)
            } else {
                format!("{} {} {text}", don.money.amount, don.money.currency)
            }
        }
        Payload::Sub(sub) => fragments_text(&sub.fragments),
        Payload::Follow(follow) => follow.display_name.clone(),
        Payload::Raid(raid) => raid.from_display_name.clone(),
        Payload::ViewerCount(item) => item.count.to_string(),
        Payload::Reward(item) => {
            let text = fragments_text(&item.fragments);
            if text.is_empty() {
                format!("{} {}", item.cost, item.title)
            } else {
                format!("{} {} {text}", item.cost, item.title)
            }
        }
        Payload::Moderation(item) => item.target_display_name.clone(),
        Payload::System(ev) => format!("{:?}", ev.code),
        Payload::Custom(custom) => custom.kind.clone(),
    }
}

fn fragments_text(fragments: &[Fragment]) -> String {
    fragments
        .iter()
        .filter_map(|fragment| match fragment {
            Fragment::Text(text) => Some(text.as_str()),
            _ => None,
        })
        .collect::<Vec<_>>()
        .join("")
}

modus_sdk::export!(Plugin);
