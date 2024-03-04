use gstd::{prelude::*, msg, collections::HashMap};
use doodles_prediction_neuronal_network_io::network::Network;
use doodles_prediction_neuronal_network_io::matrix::Matrix;
use doodles_prediction_neuronal_network_io::{
    DoodleNNAction,
    DoodleNNEvent,
    DoodleNN,
    UserData,
    InitNeuronalNetwork,
    DoodleNNStateQuery,
    DoodleNNStateReply,
    State
};

static mut NEURONAL_NETWORK: Option<DoodleNN> = None;

#[no_mangle]
extern "C" fn init() {
    let InitNeuronalNetwork {
        main_contract,
        layers
    } = msg::load()
        .expect("Error Decoding ActorId");
    unsafe {
        NEURONAL_NETWORK  = Some(DoodleNN {
            users_accepted: HashMap::new(),
            main_contract,
            owner: msg::source(),
            network: Network::new_doodle_recognition(layers),
            biases_are_set: false,
            weights_are_set: false,
            num_of_weight_to_set_data: 0,
            num_of_bias_to_set_data: 1
        });
    };
    msg::reply(DoodleNNEvent::NeuronalNetworkCreated, 0)
        .expect("Unable to reply 'XGNNEvent::NeuronalNetworkCreated'");
}

#[no_mangle]
extern "C" fn handle() {
    let action: DoodleNNAction = msg::load()
        .expect("Error decoding 'XGNNAction'");
    let state = state_mut();
    let caller = msg::source();

    match action {
        DoodleNNAction::AcceptUser(user_id) => {
            if caller != state.main_contract {
                msg::reply(DoodleNNEvent::UserIsNotTheOwner, 0)
                    .expect("Error in reply prediction");
                return;
            }
            state.users_accepted.insert(user_id, UserData {
                subscription_expired: false,
                last_prediction: Vec::new()
            });
            msg::reply(DoodleNNEvent::UserAccepted, 0)
                .expect("Error in reply prediction");
        },
        DoodleNNAction::ChangeMainContract(main_contract_id) => {
            state.main_contract = main_contract_id;

            msg::reply(DoodleNNEvent::MainContractChanged, 0)
                .expect("Error in reply prediction");
        },
        DoodleNNAction::UserSubscriptionExpired(user_id) => {
            if caller !=  state.main_contract {
                msg::reply(DoodleNNEvent::UserIsNotTheOwner, 0)
                    .expect("Error in reply prediction");
                return;
            }
            state.users_accepted
                .entry(user_id)
                .and_modify(|user_data| user_data.subscription_expired = true)
                .or_insert(UserData {
                    subscription_expired: false,
                    last_prediction: Vec::new()
                });
            msg::reply(DoodleNNEvent::SubscriptionUpdated, 0)
                .expect("Failed to reply 'XGNNEvent::SubscriptionUpdated'");
        },
        DoodleNNAction::UserSubscriptionRenewed(user_id) => {
            if caller !=  state.main_contract {
                msg::reply(DoodleNNEvent::UserIsNotTheOwner, 0)
                    .expect("Error in reply prediction");
                return;
            }
            state.users_accepted
                .entry(user_id)
                .and_modify(|user_data| user_data.subscription_expired = false)
                .or_insert(UserData {
                    subscription_expired: false,
                    last_prediction: Vec::new()
                });
            msg::reply(DoodleNNEvent::SubscriptionUpdated, 0)
                .expect("Failed to reply 'XGNNEvent::SubscriptionUpdated'");
        },
        DoodleNNAction::SetLayers(layers) => {
            if caller != state.owner {
                msg::reply(DoodleNNEvent::UserIsNotTheOwner, 0)
                    .expect("Failed to reply");
                return;
            }

            state.network.layers = layers.into_iter()
                .map(|layer| layer as usize)
                .collect();

            msg::reply(DoodleNNEvent::LayersSet, 0)
                .expect("Failed to reply 'XGNNEvent::SubscriptionUpdated'");
        },
        DoodleNNAction::SetAllWeights(weights_list) => {
            if caller != state.owner {
                msg::reply(DoodleNNEvent::UserIsNotTheOwner, 0)
                    .expect("Failed to reply");
                return;
            }

            state.network.weights = Vec::new();

            weights_list.into_iter()
                .for_each(|weights| {
                    state.network.weights.push(
                        Matrix::from(weights)
                    );
                });
            
            state.weights_are_set = true;

            msg::reply(DoodleNNEvent::WeightsSet, 0)
                .expect("Failed to reply 'XGNNEvent::SubscriptionUpdated'");
        },
        DoodleNNAction::SetAllBiases(biases_list) => {
            if caller != state.owner {
                msg::reply(DoodleNNEvent::UserIsNotTheOwner, 0)
                    .expect("Failed to reply");
                return;
            }

            state.network.biases = Vec::new();

            biases_list.into_iter()
                .for_each(|biases| {
                    state.network.biases.push(
                        Matrix::from(biases)
                    );
                });

            state.biases_are_set = true;

            msg::reply(DoodleNNEvent::BiasesSet, 0)
                .expect("Failed to reply 'XGNNEvent::SubscriptionUpdated'");
        },
        DoodleNNAction::SetWeights(data) => {
            if caller != state.owner {
                msg::reply(DoodleNNEvent::UserIsNotTheOwner, 0)
                    .expect("Failed to reply");
                return;
            }
            
            let index_weight = state.num_of_weight_to_set_data as usize;

            if index_weight > state.network.layers.len() - 1 {
                msg::reply(DoodleNNEvent::CanNotSetMoreData, 0)
                    .expect("Failed to reply");
                return;
            }

            let total_rows = state.network.layers[index_weight + 1] as usize;
            let total_cols = state.network.layers[index_weight] as usize;
            state.network.weights[index_weight].set_rows_and_cols(total_rows, total_cols);

            for fractions_data in data.into_iter() {
                if fractions_data.len() < total_cols || fractions_data.len() > total_cols {
                    panic!("Input data is not equal to the weight index");
                }
                
                state.network.weights[index_weight].add_data(
                    fractions_data
                );
            }

            if state.network.weights[index_weight].data.len() >= state.network.layers[index_weight + 1] {
                
            }

            state.weights_are_set = true;

            msg::reply(DoodleNNEvent::WeightsSet, 0)
                .expect("Error in reply");
        },
        DoodleNNAction::AddDataInNextWeitght => {
            if caller != state.owner {
                msg::reply(DoodleNNEvent::UserIsNotTheOwner, 0)
                    .expect("Failed to reply");
                return;
            }

            let index_weight = state.num_of_weight_to_set_data as usize;
            
            if state.network.weights[index_weight].data.len() < state.network.layers[index_weight + 1] {
                msg::reply(DoodleNNEvent::CanNotChangeWeightMissingData, 0)
                    .expect("Failed to reply");
                return;
            }

            state.num_of_weight_to_set_data += 1;

            if index_weight >= state.network.layers.len() - 1 {
                msg::reply(DoodleNNEvent::CanNotSetMoreData, 0)
                    .expect("Failed to reply");
                return;
            }

            msg::reply(DoodleNNEvent::ChangedIndexToChangeData, 0)
                .expect("Error in reply");
        },
        DoodleNNAction::SetBiases(data) => {
            if caller != state.owner {
                msg::reply(DoodleNNEvent::UserIsNotTheOwner, 0)
                    .expect("Failed to reply");
                return;
            }
            
            let index_bias = state.num_of_bias_to_set_data as usize;

            if index_bias > state.network.layers.len() {
                msg::reply(DoodleNNEvent::CanNotSetMoreData, 0)
                    .expect("Failed to reply");
                return;
            }

            let total_rows = state.network.layers[index_bias] as usize;
            let total_cols = 1;
            state.network.biases[index_bias-1].set_rows_and_cols(total_rows, total_cols);

            for fractions_data in data.into_iter() {
                state.network.biases[index_bias].add_data(
                    fractions_data
                );
            }

            if state.network.biases[index_bias-1].data.len() >= state.network.layers[index_bias] {
                
            }

            state.biases_are_set = true;

            msg::reply(DoodleNNEvent::BiasesSet, 0)
                .expect("Error in reply");
        },
        DoodleNNAction::AddDataInNextBias => {
            if caller != state.owner {
                msg::reply(DoodleNNEvent::UserIsNotTheOwner, 0)
                    .expect("Failed to reply");
                return;
            }

            let index_bias = state.num_of_bias_to_set_data as usize;
            
            if state.network.biases[index_bias-1].data.len() < state.network.layers[index_bias] {
                msg::reply(DoodleNNEvent::CanNotChangeWeightMissingData, 0)
                    .expect("Failed to reply");
                return;
            }

            state.num_of_bias_to_set_data += 1;

            if index_bias >= state.network.layers.len() {
                msg::reply(DoodleNNEvent::CanNotSetMoreData, 0)
                    .expect("Failed to reply");
                return;
            }

            msg::reply(DoodleNNEvent::ChangedIndexToChangeData, 0)
                .expect("Error in reply");
        },
        DoodleNNAction::Predict(doodle_image) => {

            // Needs to change all contract to set a general actions and events
            // for the moment, this restrictions need to be commented

            // if !state.users_accepted.get(&caller).is_some() {
            //     msg::reply(DoodleNNEvent::UserIsNotSubscribed, 0)
            //         .expect("Failed to reply 'XGNNEvent::UserIsNotSubscribed'");
            //     return;
            // }

            // if state.users_accepted.get(&caller).unwrap().subscription_expired{
            //     msg::reply(DoodleNNEvent::SubscriptionExpired, 0)
            //         .expect("Failed to reply 'XGNNEvent::SubscriptionExpired'");
            //     return;
            // }

            if !state.biases_are_set || !state.weights_are_set {
                msg::reply(DoodleNNEvent::NNIsNotComplete, 0)
                    .expect("Error sending reply");
                return;
            }
            
            let prediction = state.network.feed_forward(doodle_image);

            let response: Vec<String> = prediction.iter().map(|fraction| fraction.to_string()).collect();

            state.users_accepted
                .entry(caller)
                .and_modify(|user_data| user_data.last_prediction = response.clone())
                .or_insert(UserData {
                    subscription_expired: false,
                    last_prediction: response.clone()
                });

            msg::reply(DoodleNNEvent::Prediction(response), 0)
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
        DoodleNNStateQuery::SubscriptionExpired(user) => {
            if !state.users_accepted.get(&user).is_some() {
                msg::reply(DoodleNNStateReply::UserIsNotSubscribed, 0)
                    .expect("Failed to reply 'XGNNEvent::UserIsNotSubscribed'");
                return;
            }

            let user_data = state.users_accepted.get(&user).unwrap();

            msg::reply(DoodleNNStateReply::SubscriptionExpired(user_data.subscription_expired), 0) 
                .expect("Error sending state reply");
        },
        DoodleNNStateQuery::UserIsSubscribed(user) => {
            let user_is_subscribed = state.users_accepted.get(&user).is_some();

            msg::reply(DoodleNNStateReply::UserIsSubscribed(user_is_subscribed), 0) 
                .expect("Error sending state reply");
        },
        DoodleNNStateQuery::UserLastPrediction(user) => {
            if !state.users_accepted.get(&user).is_some() {
                msg::reply(DoodleNNStateReply::UserIsNotSubscribed, 0)
                    .expect("Failed to reply 'XGNNEvent::UserIsNotSubscribed'");
                return;
            }

            let user_data = state.users_accepted.get(&user).unwrap();

            msg::reply(DoodleNNStateReply::UserLastPrediction(user_data.last_prediction.clone()), 0) 
                .expect("Error sending state reply");
        },
        DoodleNNStateQuery::All => {
            let doodle_nn = state_ref();
            let response:State = doodle_nn.into();
            msg::reply::<DoodleNNStateReply>(DoodleNNStateReply::All(response), 0)
                .expect("Failed to encode or reply with `State` from `state()`");
            // msg::reply(((*state_mut().clone()).into()), 0) 
            //     .expect("Error sending state reply");
        }
    }
}

fn state_mut() -> &'static mut DoodleNN {
    let state = unsafe { NEURONAL_NETWORK.as_mut() };
    debug_assert!(state.is_some(), "State isn't initialized");
    unsafe { state.unwrap_unchecked() }
}

fn state_ref() -> &'static DoodleNN {
    let state = unsafe { NEURONAL_NETWORK.as_ref() };
    debug_assert!(state.is_some(), "State isn't initialized");
    unsafe { state.unwrap_unchecked() }
}