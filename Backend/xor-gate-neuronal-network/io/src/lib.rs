#![no_std]
use gstd::{prelude::*, Vec, ActorId, HashMap};
use gmeta::{Metadata, In, Out, InOut};

pub mod network;
pub mod matrix;
pub mod exponentiation_str;
pub mod division_str;
pub mod multiplication_str;
pub mod sums_str;
pub mod subtraction_str;
pub mod utils;

use network::Network; 

pub struct ProgramMetadata;

pub type Predict = (BinaryLogic, BinaryLogic);
pub type UserId = ActorId;
pub type Expired = bool;

impl Metadata for ProgramMetadata {
    type Init = InOut<ActorId,LogicGateEvent>;
    type Handle = InOut<LogicGateAction, LogicGateEvent>;
    type Reply = ();
    type Others = ();
    type Signal = ();
    type State = Out<State>;
}

pub struct LogicGate {
    pub users_accepted: HashMap<ActorId, Expired>,
    pub main_contract: ActorId,
    pub network: Network
}

#[derive(Encode, Decode, TypeInfo)]
pub enum LogicGateAction {
    AcceptUser(ActorId),
    UserSubscriptionExpired(ActorId),
    UserSubscriptionRenewed(ActorId),
    Predict((BinaryLogic, BinaryLogic)),
}

#[derive(Encode, Decode, TypeInfo)]
pub enum LogicGateEvent {
    NeuronalNetworkCreated,
    UserIsNotTheOwner,
    UserIsNotSubscribed,
    SubscriptionUpdated,
    SubscriptionExpired,
    UserAccepted,
    Prediction(Vec<String>)   
}

#[derive(Encode, Decode, TypeInfo)]
pub struct State {
    pub users_accepted: Vec<(ActorId, bool)>,
    pub main_contract: ActorId,
    pub network: Network  
}

#[derive(Encode, Decode, TypeInfo)]
pub enum BinaryLogic {
    One,
    Zero
}

impl From<LogicGate> for State {
    fn from(value: LogicGate) -> Self {
        let LogicGate {
            users_accepted,
            main_contract,
            network
        } = value;
        
        let users_accepted = users_accepted
            .iter()
            .map(|(user, accepted)| (*user, *accepted))
            .collect();
            
        Self {
            users_accepted,
            main_contract,
            network
        }
    }
} 


