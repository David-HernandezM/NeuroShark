#![no_std]
use gstd::{prelude::*, Vec, ActorId, collections::HashMap};
use gmeta::{Metadata, InOut};

pub mod fraction;
pub mod matrix;
pub mod network;
pub mod utils;

use fraction::Fraction;
use network::Network; 

pub type DoodleImage = Vec<Fraction>;

pub struct ProgramMetadata;

pub type UserId = ActorId;
pub type Expired = bool;

impl Metadata for ProgramMetadata {
    type Init = InOut<InitNeuronalNetwork, DoodleNNEvent>;
    type Handle = InOut<DoodleNNAction, DoodleNNEvent>;
    type Reply = ();
    type Others = ();
    type Signal = ();
    type State = InOut<DoodleNNStateQuery, DoodleNNStateReply>;
}

#[derive(Encode, Decode, TypeInfo, Clone)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct InitNeuronalNetwork {
    pub main_contract: ActorId,
    pub layers: Vec<u64>
}

#[derive(Encode, Decode, TypeInfo, Clone)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct UserData {
    pub subscription_expired: Expired,
    pub last_prediction: Vec<String>
}

pub struct DoodleNN {
    pub users_accepted: HashMap<ActorId, UserData>,
    pub main_contract: ActorId,
    pub owner: ActorId,
    pub network: Network,
    pub biases_are_set: bool,
    pub num_of_weight_to_set_data: u64,
    pub num_of_bias_to_set_data: u64,
    pub weights_are_set: bool,
}

#[derive(Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum DoodleNNAction {
    AcceptUser(ActorId),
    ChangeMainContract(ActorId),
    UserSubscriptionExpired(ActorId),
    UserSubscriptionRenewed(ActorId),
    SetLayers(Vec<u64>),
    SetAllWeights(Vec<Vec<Vec<Fraction>>>),
    SetAllBiases(Vec<Vec<Vec<Fraction>>>),
    SetWeights(Vec<Vec<Fraction>>),
    AddDataInNextWeitght,
    SetBiases(Vec<Vec<Fraction>>),
    AddDataInNextBias,
    Predict(DoodleImage),
}

#[derive(Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum DoodleNNEvent {
    NeuronalNetworkCreated,
    MainContractChanged,
    UserIsNotTheOwner,
    UserIsNotSubscribed,
    SubscriptionUpdated,
    SubscriptionExpired,
    UserAccepted,
    LayersSet,
    WeightsSet,
    BiasesSet,
    CanNotChangeWeightMissingData,
    CanNotChangeBiasMissingData,
    WeightChanged,
    BiasChanged,
    ChangedIndexToChangeData,
    CanNotSetMoreData,
    NNIsNotComplete,
    Prediction(Vec<String>)
}

#[derive(Encode, Decode, TypeInfo, Clone)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum DoodleNNStateQuery {
    SubscriptionExpired(ActorId),
    UserIsSubscribed(ActorId),
    UserLastPrediction(ActorId),
    All
}

#[derive(Encode, Decode, TypeInfo, Clone)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum DoodleNNStateReply {
    SubscriptionExpired(bool),
    UserLastPrediction(Vec<String>),
    UserIsNotSubscribed,
    UserIsSubscribed(bool),
    All(State)
}

#[derive(Encode, Decode, TypeInfo, Clone)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct NetworkState {
    pub layers: Vec<u64>,
    pub weights: MatrixState,
    pub biases: MatrixState,
    pub weights_size: Vec<u64>,
    pub biases_size: Vec<u64>,
}

#[derive(Encode, Decode, TypeInfo, Clone)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct MatrixState {
    pub rows: u64,
    pub cols: u64,
    pub data_rows: u64,
    pub data_cols: u64
}

#[derive(Encode, Decode, TypeInfo, Clone)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct State {
    pub users_accepted: Vec<(ActorId, UserData)>,
    pub main_contract: ActorId,
    pub network: String,
    pub weights_are_set: bool,
    pub biases_are_set: bool,
    pub owner: ActorId
}

impl From<&DoodleNN> for State {
    fn from(value: &DoodleNN) -> Self {
        let DoodleNN {
            users_accepted,
            main_contract,
            weights_are_set,
            biases_are_set,
            owner,
            ..
        } = value;
        
        let users_accepted = users_accepted
            .clone()
            .iter()
            .map(|(user, user_data)| (*user, user_data.clone()))
            .collect();
            
        Self {
            users_accepted,
            main_contract: *main_contract,
            network: String::from("Doodle image detection neuronal network"),
            weights_are_set: *weights_are_set,
            biases_are_set: *biases_are_set,
            owner: *owner
        }
    }
} 


