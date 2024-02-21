use gstd::{prelude::*, msg, ActorId};
use program_io::network::Network;
use program_io::matrix::Matrix;
use program_io::{
    LogicGateAction,
    LogicGateEvent,
    BinaryLogic,
    LogicGate,
    Predict,
    State
};

static mut NEURONAL_NETWORK: Option<LogicGate> = None;

#[no_mangle]
extern "C" fn init() {
    let owner: ActorId = msg::load()
        .expect("Error Decoding ActorId");
    unsafe {
        NEURONAL_NETWORK  = Some(LogicGate {
            users_accepted: HashMap::new(),
            main_contract: owner,
            network: Network::new()
        });
    };
    msg::reply(LogicGateEvent::NeuronalNetworkCreated, 0)
        .expect("Unable to reply 'XGNNEvent::NeuronalNetworkCreated'");
}

#[no_mangle]
extern "C" fn handle() {
    let action: LogicGateAction = msg::load()
        .expect("Error decoding 'XGNNAction'");
    let xgnn = state_mut();
    
    match action {
        LogicGateAction::AcceptUser(user_id) => {
            let user_id = msg::source();
            if user_id !=  xgnn.main_contract {
                msg::reply(LogicGateEvent::UserIsNotTheOwner, 0)
                    .expect("Error in reply prediction");
                return;
            }
            xgnn.users_accepted.insert(user_id, false);
            msg::reply(LogicGateEvent::UserAccepted, 0)
                .expect("Error in reply prediction");
        },
        LogicGateAction::UserSubscriptionExpired(user_id) => {
            if msg::source() !=  xgnn.main_contract {
                msg::reply(LogicGateEvent::UserIsNotTheOwner, 0)
                    .expect("Error in reply prediction");
                return;
            }
            xgnn.users_accepted
                .entry(user_id)
                .and_modify(|expired| *expired = true)
                .or_insert(false);
            msg::reply(LogicGateEvent::SubscriptionUpdated, 0)
                .expect("Failed to reply 'XGNNEvent::SubscriptionUpdated'");
        },
        LogicGateAction::UserSubscriptionRenewed(user_id) => {
            if msg::source() !=  xgnn.main_contract {
                msg::reply(LogicGateEvent::UserIsNotTheOwner, 0)
                    .expect("Error in reply prediction");
                return;
            }
            xgnn.users_accepted
                .entry(user_id)
                .and_modify(|expired| *expired = false)
                .or_insert(true);
            msg::reply(LogicGateEvent::SubscriptionUpdated, 0)
                .expect("Failed to reply 'XGNNEvent::SubscriptionUpdated'");
        },
        LogicGateAction::Predict(data) => {
            if xgnn.users_accepted.get(&msg::source()) == None {
                msg::reply(LogicGateEvent::UserIsNotSubscribed, 0)
                    .expect("Failed to reply 'XGNNEvent::UserIsNotSubscribed'");
                return;
            }
            if *xgnn.users_accepted.get(&msg::source()).unwrap() {
                msg::reply(LogicGateEvent::SubscriptionExpired, 0)
                    .expect("Failed to reply 'XGNNEvent::SubscriptionExpired'");
                return;
            }
            
            let val1 = match data.0 {
                BinaryLogic::One => String::from("1.0"),
                _ => String::from("0.0")
            };
            let val2 = match data.1 {
                BinaryLogic::One => String::from("1.0"),
                _ => String::from("0.0")
            };
            let inputs = vec![val1, val2];
            
            let prediction = xgnn.network.feed_forward(inputs);
            msg::reply(LogicGateEvent::Prediction(prediction), 0)
                .expect("Error in reply prediction");
        }
    }
}

#[no_mangle]
extern "C" fn state() {
    let xgnn = unsafe { NEURONAL_NETWORK.take().expect("Unexpected error in taking state")};
    msg::reply::<State>(xgnn.into(), 0)
        .expect("Failed to encode or reply with `State` from `state()`");
}

fn state_mut() -> &'static mut LogicGate {
    let state = unsafe { NEURONAL_NETWORK.as_mut() };
    debug_assert!(state.is_some(), "State isn't initialized");
    unsafe { state.unwrap_unchecked() }
}