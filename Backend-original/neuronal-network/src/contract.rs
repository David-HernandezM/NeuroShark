use gstd::{prelude::*, msg};
use program_io::network::Network;
use program_io::matrix::Matrix;
use program_io::{
    NNMessageIn,
    NNMessageOut,
    NNStructure,
    BinaryLogic
};

static mut NEURONAL_NETWORK: Option<Network> = None;

#[no_mangle]
extern "C" fn init() {
    let init_msg: NNMessageIn = msg::load()
        .expect("Error loading init message");
    
    match init_msg {
        NNMessageIn::SetLogicalXorNeuronalNetwork => {
            set_logic_xor_nn();
            msg::reply(NNMessageOut::NeuronalNetworkCreated, 0)
                .expect("Error in reply");
        },
        NNMessageIn::SetDefaultNeuronalNetwork => {
            unsafe {
                NEURONAL_NETWORK = Some(Network::default_new());
            };
            msg::reply(NNMessageOut::DefaultNeuronalNetworkCreated, 0)
                .expect("Error in reply");
        },
        NNMessageIn::SetNewTrainedNeuronalNetwork(nn_data) => {
            set_nn_data(nn_data);
            msg::reply(NNMessageOut::NeuronalNetworkCreated, 0)
                .expect("Error in reply");
        },
        _ => {
            unsafe {
                NEURONAL_NETWORK = Some(Network::default_new());
            };
            msg::reply(
                NNMessageOut::ErrorCreatingNeuronalNetwork(
                    String::from("Invalid option, default neuronal network created")
                ), 
                0
            ).expect("Error in reply");
        }
    }
}

#[no_mangle]
extern "C" fn handle() {
    let message: NNMessageIn = msg::load()
        .expect("Error loading init message");
    match message {
        NNMessageIn::SetLogicalXorNeuronalNetwork => {
            set_logic_xor_nn();
            msg::reply(NNMessageOut::NeuronalNetworkCreated, 0)
                .expect("Error in reply");
        },
        NNMessageIn::PredictResultLogicalXor(data) => {
            let state = state_mut();
            let val1 = match data.0 {
                BinaryLogic::One => String::from("1.0"),
                _ => String::from("0.0")
            };
            let val2 = match data.1 {
                BinaryLogic::One => String::from("1.0"),
                _ => String::from("0.0")
            };
            let inputs = vec![val1, val2];
            
            let prediction = state.feed_forward(inputs);
            msg::reply(NNMessageOut::Prediction(prediction), 0)
                .expect("Error in reply prediction");
        },
        NNMessageIn::SetDefaultNeuronalNetwork => {
            unsafe {
                NEURONAL_NETWORK = Some(Network::default_new());
            };
            msg::reply(NNMessageOut::DefaultNeuronalNetworkCreated, 0)
                .expect("Error in reply");
        },
        NNMessageIn::SetNewTrainedNeuronalNetwork(nn_data) => {
            set_nn_data(nn_data);
            msg::reply(NNMessageOut::EstablishedNeuronalNetworkData, 0)
                .expect("Error in reply");
        },
        NNMessageIn::SetTrainedWeightsMatrix(weights) => {
            let state = state_mut();
            state.weights = weights
                .into_iter()
                .map(|weight| {
                    Matrix::from(weight)
                })
                .collect();
            //state.weights = vec![Matrix::from(weights)];
            msg::reply(NNMessageOut::EstablishedWeightMatrix, 0).expect("Error in reply");
        },
        
        NNMessageIn::SetTrainedBiasMatrix(biases) => {
            let state = state_mut();
            state.biases = biases
                .into_iter()
                .map(|bias| {
                    Matrix::from(bias)
                })
                .collect();
            //state.biases = Matrix::from(biases);
            msg::reply(NNMessageOut::EstablishedBiasMatrix, 0).expect("Error in reply");
        },
        /*
        NNMessageIn::PredictResultOf(inputs) => {
            let state = state_mut();
            let result = state.feed_forward(inputs);
            msg::reply(NNMessageOut::Prediction(result), 0).expect("Error in reply");
        }
        */
    }
}

#[no_mangle]
extern "C" fn state() {
    let state = state_ref();
    msg::reply(state, 0).expect("Error in sending state");
}

fn state_mut() -> &'static mut Network {
    let state = unsafe { NEURONAL_NETWORK.as_mut() };
    debug_assert!(state.is_some(), "State isn't initialized");
    unsafe { state.unwrap_unchecked() }
}

fn state_ref() -> &'static Network {
    let state = unsafe { NEURONAL_NETWORK.as_ref() };
    debug_assert!(state.is_some(), "State isn't initialized");
    unsafe { state.unwrap_unchecked() }
}

fn set_nn_data(nn_data: NNStructure) {
    unsafe {
        NEURONAL_NETWORK = Some(Network::new(
            nn_data.layers,
            nn_data.weights,
            nn_data.biases,
            nn_data.learning_rate,
        ));
    };
}

fn set_logic_xor_nn() {
    let state = state_mut();
    let learning_rate = String::from("0.5");
    let layers: Vec<u64> = vec![2, 3, 1];
    let weights = vec![
    	Matrix::from(vec![
        	vec![
            	String::from("-10.067806895028"),
            	String::from("-10.069251807067"),
        	],
        	vec![
            	String::from("-9.382214114334"),
            	String::from("-9.383488644351"),
        	],
        	vec![
            	String::from("-10.195950770812"),
            	String::from("-10.195385621153"),
        	],
    	]),
    	Matrix::from(vec![
        	vec![
            	String::from("-5.203677476184"),
            	String::from("-4.022237746981"),
            	String::from("8.569942138251"),
        	],
    	]),
	];
	
	let biases = vec![
    	Matrix::from(vec![
        	vec![
            	String::from("4.457786935168"),
        	],
        	vec![
            	String::from("4.111367041253"),
        	],
        	vec![
            	String::from("15.282025368793"),
        	],
    	]),
    	Matrix::from(vec![
        	vec![
            	String::from("-4.083288690267"),
        	],
    	]),
	];
	
    state.layers = layers;
    state.weights = weights;
    state.biases = biases;
    state.learning_rate = learning_rate;
}





/*
use activations::SIGMOID;
use network::Network;

pub mod activations;
pub mod matrix;
pub mod network;

fn main() {
	let inputs = vec![
		vec![0.0, 0.0],
		vec![0.0, 1.0],
		vec![1.0, 0.0],
		vec![1.0, 1.0],
	];
	let targets = vec![vec![0.0], vec![1.0], vec![1.0], vec![0.0]];

	let mut network = Network::new(vec![2, 3, 1], 0.5, SIGMOID);

	println!("{:?}", network.feed_forward(vec![0.0, 0.0]));
	println!("{:?}", network.feed_forward(vec![0.0, 1.0]));
	println!("{:?}", network.feed_forward(vec![1.0, 0.0]));
	println!("{:?}", network.feed_forward(vec![1.0, 1.0]));
	
	network.train(inputs, targets, 20000);

	println!("{:?}", network.feed_forward(vec![0.0, 0.0]));
	println!("{:?}", network.feed_forward(vec![0.0, 1.0]));
	println!("{:?}", network.feed_forward(vec![1.0, 0.0]));
	println!("{:?}", network.feed_forward(vec![1.0, 1.0]));
	println!("\n layers {:?}", network.biases);
	println!("\n Weights {:?}", network.weights);
	println!("\n biases {:?}", network.biases);
	println!("\n data {:?}", network.data);
	println!("\n learning_rate {:?}", network.learning_rate);
}

*/