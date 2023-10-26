#![no_std]
use gstd::{prelude::*, Vec};
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

impl Metadata for ProgramMetadata {
    type Init = In<NNMessageIn>;
    type Handle = InOut<NNMessageIn, NNMessageOut>;
    type Reply = ();
    type Others = ();
    type Signal = ();
    type State = Out<Network>;
}

#[derive(Encode, Decode, TypeInfo)]
pub enum NNMessageIn {
    SetLogicalXorNeuronalNetwork,
    PredictResultLogicalXor((BinaryLogic, BinaryLogic)),
    SetDefaultNeuronalNetwork,
    SetNewTrainedNeuronalNetwork(NNStructure),
    SetTrainedWeightsMatrix(Vec<Vec<Vec<String>>>),
    SetTrainedBiasMatrix(Vec<Vec<Vec<String>>>),
    //PredictResultOf(Vec<String>),
}

#[derive(Encode, Decode, TypeInfo)]
pub enum NNMessageOut {
    NeuronalNetworkCreated,
    DefaultNeuronalNetworkCreated,
    EstablishedNeuronalNetworkData,
    EstablishedWeightMatrix,
    EstablishedBiasMatrix,
    Prediction(Vec<String>),
    ErrorCreatingNeuronalNetwork(String),
    ErrorSettingTrainedBias(String),
    ErrorSettingTrainedWeights(String),
}

#[derive(Encode, Decode, TypeInfo)]
pub struct NNStructure {
    pub layers: Vec<u64>,
	pub weights: Vec<Vec<Vec<String>>>,
	pub biases: Vec<Vec<Vec<String>>>,
	pub learning_rate: String,
}

#[derive(Encode, Decode, TypeInfo)]
pub enum BinaryLogic {
    One,
    Zero
}
