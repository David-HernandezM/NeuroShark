use gstd::{prelude::*, msg, ActorId, collections::HashMap};
use xor_gate_neuronal_network_io::network::Network;
use xor_gate_neuronal_network_io::fraction::Fraction;
use xor_gate_neuronal_network_io::{
    LogicGateAction,
    LogicGateEvent,
    BinaryLogic,
    LogicGate,
    State,
    UserData,
    LogicNNStateQuery,
    LogicNNStateReply
};

static mut NEURONAL_NETWORK: Option<LogicGate> = None;

#[no_mangle]
extern "C" fn init() {
    let main_contract: ActorId = msg::load()
        .expect("Error Decoding ActorId");
    unsafe {
        NEURONAL_NETWORK  = Some(LogicGate {
            users_accepted: HashMap::new(),
            main_contract: main_contract,
            owner: msg::source(),
            network: Network::new_xor_gate()
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
    let caller = msg::source();
    
    match action {
        LogicGateAction::AcceptUser(user_id) => {
            if caller != xgnn.main_contract && caller != xgnn.owner {
                msg::reply(LogicGateEvent::UserIsNotTheOwner, 0)
                    .expect("Error in reply prediction");
                return;
            }
            xgnn.users_accepted.insert(user_id, UserData {
                subscription_expired: false,
                last_prediction: String::from("")
            });
            msg::reply(LogicGateEvent::UserAccepted, 0)
                .expect("Error in reply prediction");
        },
        LogicGateAction::UserSubscriptionExpired(user_id) => {
            if caller != xgnn.main_contract && caller != xgnn.owner {
                msg::reply(LogicGateEvent::UserIsNotTheOwner, 0)
                    .expect("Error in reply prediction");
                return;
            }

            xgnn.users_accepted
                .entry(user_id)
                .and_modify(|user_data| user_data.subscription_expired = true)
                .or_insert(UserData {
                    subscription_expired: true,
                    last_prediction: String::from("")
                });
            msg::reply(LogicGateEvent::SubscriptionUpdated, 0)
                .expect("Failed to reply 'XGNNEvent::SubscriptionUpdated'");
        },
        LogicGateAction::UserSubscriptionRenewed(user_id) => {
            if caller != xgnn.main_contract && caller != xgnn.owner {
                msg::reply(LogicGateEvent::UserIsNotTheOwner, 0)
                    .expect("Error in reply prediction");
                return;
            }

            xgnn.users_accepted
                .entry(user_id)
                .and_modify(|user_data| user_data.subscription_expired = false)
                .or_insert(UserData {
                    subscription_expired: false,
                    last_prediction: String::from("")
                });
            msg::reply(LogicGateEvent::SubscriptionUpdated, 0)
                .expect("Failed to reply 'XGNNEvent::SubscriptionUpdated'");
        },
        LogicGateAction::ChangeMainContract(new_address) => {
            if caller != xgnn.owner {
                msg::reply(LogicGateEvent::UserIsNotTheOwner, 0)
                    .expect("Error in reply prediction");
                return;
            }

            xgnn.main_contract = new_address;

            msg::reply(LogicGateEvent::MainContractChanged, 0)
                .expect("Failed to reply 'XGNNEvent::SubscriptionUpdated'");
        },
        LogicGateAction::Predict(data) => {
            if !xgnn.users_accepted.get(&caller).is_some() {
                msg::reply(LogicGateEvent::UserIsNotSubscribed, 0)
                    .expect("Failed to reply 'XGNNEvent::UserIsNotSubscribed'");
                return;
            }
            if xgnn.users_accepted.get(&caller).unwrap().subscription_expired {
                msg::reply(LogicGateEvent::SubscriptionExpired, 0)
                    .expect("Failed to reply 'XGNNEvent::SubscriptionExpired'");
                return;
            }
            
            let val1 = match data.0 {
                BinaryLogic::One => Fraction::new_from_int(1),// String::from("1.0"),
                _ => Fraction::new_from_int(0) // String::from("0.0")
            };
            let val2 = match data.1 {
                BinaryLogic::One => Fraction::new_from_int(1),// String::from("1.0"),
                _ => Fraction::new_from_int(0) // String::from("0.0")
            };

            let inputs = vec![val1, val2];
            
            let prediction = xgnn.network.feed_forward(inputs);

            let response = format!("{} / {}", prediction[0].num, prediction[0].den);

            xgnn.users_accepted
                .entry(caller)
                .and_modify(|user_data| user_data.last_prediction = response.clone())
                .or_insert(UserData {
                    subscription_expired: false,
                    last_prediction: String::from("")
                });

            msg::reply(LogicGateEvent::Prediction(response), 0)
                .expect("Error in reply prediction");
        }
    }
}

#[no_mangle]
extern "C" fn state() {
    let message = msg::load()
        .expect("Error loading message");
    let state = state_mut();

    match message {
        LogicNNStateQuery::SubscriptionExpired(user) => {
            if !state.users_accepted.get(&user).is_some() {
                msg::reply(LogicNNStateReply::UserIsNotSubscribed, 0)
                    .expect("Failed to reply 'XGNNEvent::UserIsNotSubscribed'");
                return;
            }

            let user_data = state.users_accepted.get(&user).unwrap();

            msg::reply(LogicNNStateReply::SubscriptionExpired(user_data.subscription_expired), 0) 
                .expect("Error sending state reply");
        },
        LogicNNStateQuery::UserIsSubscribed(user) => {
            let user_is_subscribed = state.users_accepted.get(&user).is_some();

            msg::reply(LogicNNStateReply::UserIsSubscribed(user_is_subscribed), 0) 
                .expect("Error sending state reply");
        },
        LogicNNStateQuery::UserLastPrediction(user) => {
            if !state.users_accepted.get(&user).is_some() {
                msg::reply(LogicNNStateReply::UserIsNotSubscribed, 0)
                    .expect("Failed to reply 'XGNNEvent::UserIsNotSubscribed'");
                return;
            }

            let user_data = state.users_accepted.get(&user).unwrap();

            msg::reply(LogicNNStateReply::UserLastPrediction(user_data.last_prediction.clone()), 0) 
                .expect("Error sending state reply");
        },
        LogicNNStateQuery::All => {
            let xgnn = state_ref();
            let response: State = xgnn.into();
            msg::reply::<LogicNNStateReply>(LogicNNStateReply::All(response), 0)
                .expect("Failed to encode or reply with `State` from `state()`");
            // msg::reply(((*state_mut().clone()).into()), 0) 
            //     .expect("Error sending state reply");
        }
    }
}

fn state_mut() -> &'static mut LogicGate {
    let state = unsafe { NEURONAL_NETWORK.as_mut() };
    debug_assert!(state.is_some(), "State isn't initialized");
    unsafe { state.unwrap_unchecked() }
}

fn state_ref() -> &'static LogicGate {
    let state = unsafe { NEURONAL_NETWORK.as_ref() };
    debug_assert!(state.is_some(), "State isn't initialized");
    unsafe { state.unwrap_unchecked() }
}