use gtest::{ Log, System, Program };
use program_io::{
    NNStructure,
    NNMessageIn,
    NNMessageOut
};

#[test]
fn neuronal_network_default_init() {
    let sys = System::new();
    sys.init_logger();
    let neuronal_network = Program::current(&sys);
    
    let expected_log = Log::builder()
        .dest(2)
        .payload(NNMessageOut::DefaultNeuronalNetworkCreated);
    let res = neuronal_network.send(2, NNMessageIn::SetDefaultNeuronalNetwork);
    assert!(!res.main_failed());
    assert!(res.contains(&expected_log));
}

#[test]
fn neuronal_network_trained_init() {
    let sys = System::new();
    sys.init_logger();
    let neuronal_network = Program::current(&sys);
    
    let expected_log = Log::builder()
        .dest(2)
        .payload(NNMessageOut::NeuronalNetworkCreated);
    let res = neuronal_network.send(2, NNMessageIn::SetNewTrainedNeuronalNetwork(trained_data()));
    
    assert!(!res.main_failed());
    assert!(res.contains(&expected_log));
}

#[test] 
fn init_logic_xor_neuronal_network() {
    let sys = System::new();
    sys.init_logger();
    let neuronal_network = Program::current(&sys);
    
    let expected_log = Log::builder()
        .dest(2)
        .payload(NNMessageOut::NeuronalNetworkCreated);
    let res = neuronal_network.send(2, NNMessageIn::SetLogicalXorNeuronalNetwork);
    assert!(!res.main_failed());
    assert!(res.contains(&expected_log));
}


fn trained_data() -> NNStructure {
    let learning_rate = String::from("0.5");
    let layers: Vec<u64> = vec![2, 3, 1];
    let weights  = vec![
        vec![
            vec![
                String::from("12.50425529537867"),
                String::from("-9.01838761548488"),
            ],
            vec![
                String::from("8.16915129424116"),
                String::from("8.15134749644964"),
            ],
            vec![
                String::from("-8.32388355843063"),
                String::from("12.30078899179055"),
            ],
        ],
        vec![
            vec![
                String::from("-7.65326754223358"),
                String::from("6.23761444985032"),
                String::from("-7.65621832150449"),
            ],
        ],
    ];
    let biases = vec![
        vec![
            vec![
                String::from("3.07520074139361"),
            ],
            vec![
                String::from("-1.51949047343958"),
            ],
            vec![
                String::from("2.52278770601655"),
            ],
        ],
        vec![
            vec![
                String::from("5.36466499159904"),
            ]
        ],
    ];
    
    NNStructure {
        layers,
        weights,
        biases,
        learning_rate
    }
}

