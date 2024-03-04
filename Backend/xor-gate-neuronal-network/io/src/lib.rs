#![no_std]
use gstd::{prelude::*, Vec, ActorId, collections::HashMap};
use gmeta::{Metadata, InOut};

pub mod fraction;
pub mod matrix;
pub mod network;
pub mod utils;

use network::Network; 

pub struct ProgramMetadata;

pub type Predict = (BinaryLogic, BinaryLogic);
pub type UserId = ActorId;
pub type Expired = bool;

impl Metadata for ProgramMetadata {
    type Init = InOut<InitContract,LogicGateEvent>;
    type Handle = InOut<LogicGateAction, LogicGateEvent>;
    type Reply = ();
    type Others = ();
    type Signal = ();
    type State = InOut<LogicNNStateQuery, LogicNNStateReply>;
}

#[derive(Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct InitContract {
    owner: ActorId,
    main_contract: ActorId
}

pub struct LogicGate {
    pub owner: ActorId,
    pub users_accepted: HashMap<ActorId, UserData>,
    pub main_contract: ActorId,
    pub network: Network
}

#[derive(Encode, Decode, TypeInfo, Clone)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct UserData {
    pub subscription_expired: Expired,
    pub last_prediction: String
}

#[derive(Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum LogicGateAction {
    AcceptUser(ActorId),
    UserSubscriptionExpired(ActorId),
    UserSubscriptionRenewed(ActorId),
    ChangeMainContract(ActorId),
    Predict((BinaryLogic, BinaryLogic)),
}

#[derive(Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum LogicGateEvent {
    NeuronalNetworkCreated,
    UserIsNotTheOwner,
    UserIsNotSubscribed,
    SubscriptionUpdated,
    SubscriptionExpired,
    UserAccepted,
    MainContractChanged,
    Prediction(String) // Prediction(Vec<String>)   
}

#[derive(Encode, Decode, TypeInfo, Clone)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum LogicNNStateQuery {
    SubscriptionExpired(ActorId),
    UserIsSubscribed(ActorId),
    UserLastPrediction(ActorId),
    All
}

#[derive(Encode, Decode, TypeInfo, Clone)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum LogicNNStateReply {
    SubscriptionExpired(bool),
    UserLastPrediction(String),
    UserIsSubscribed(bool),
    UserIsNotSubscribed,
    All(State)
}

#[derive(Encode, Decode, TypeInfo, Clone)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct State {
    pub users_accepted: Vec<(ActorId, UserData)>,
    pub owner: ActorId,
    pub main_contract: ActorId,
    pub network: String //Network  
}

#[derive(Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum BinaryLogic {
    One,
    Zero
}

impl From<&LogicGate> for State {
    fn from(value: &LogicGate) -> Self {
        let LogicGate {
            users_accepted,
            main_contract,
            owner,
            ..
        } = value;
        
        let users_accepted = users_accepted
            .iter()
            .map(|(user, user_data)| (*user, user_data.clone()))
            .collect();
            
        Self {
            users_accepted,
            main_contract: *main_contract,
            owner: *owner,
            network: String::from("Xor neuronal network")
        }
    }
} 


