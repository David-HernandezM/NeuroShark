use gstd::{prelude::*, msg};
use ping_pong_io::*;

static mut CONTRACT: Option<PingStateType> = None;

#[no_mangle]
extern "C" fn init() {
    unsafe {
        CONTRACT = Some(Vec::new());
    };

    msg::reply("Contract initialized", 0)
        .expect("Error sending reply");
}

#[no_mangle]
extern "C" fn handle() {
    let message_type: PingAction = msg::load()
        .expect("Error loading message");
    let caller = msg::source();
    let contract = mut_state();

    match message_type {
        PingAction::Ping => {
            contract.push((caller, PingAction::Ping));
            msg::reply(PingEvent::Pong, 0)
                .expect("Error sending reply 'PingState::Pong'");
        },
        PingAction::Pong => {
            contract.push((caller, PingAction::Pong));
            msg::reply(PingEvent::Ping, 0)
            .expect("Error sending reply 'PingState::Ping'");
        }
    }
}

#[no_mangle]
extern "C" fn state() {
    msg::reply(mut_state(), 0)
        .expect("Error sending state");
} 

fn mut_state() -> &'static mut PingStateType {
    let state = unsafe { CONTRACT.as_mut() };
    debug_assert!(state.is_some(), "State it not initialized");
    unsafe { state.unwrap_unchecked() }
}
