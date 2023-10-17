use gtest::{ Log, System, Program };
use program_io::{
    NNMessageIn,
    NNMessageOut,
    BinaryLogic
};

#[test] 
fn prediction_test_0_0() {
    let sys = System::new();
    sys.init_logger();
    let neuronal_network = Program::current(&sys);
    
    let mut expected_log = Log::builder()
        .dest(2)
        .payload(NNMessageOut::NeuronalNetworkCreated);
    let mut res = neuronal_network.send(2, NNMessageIn::SetLogicalXorNeuronalNetwork);
    assert!(!res.main_failed());
    assert!(res.contains(&expected_log));
    
    
    expected_log = Log::builder()
        .dest(2)
        .payload(NNMessageOut::Prediction(vec![String::from("0.009806929001")]));
    res = neuronal_network.send(2, NNMessageIn::PredictResultLogicalXor((
        BinaryLogic::Zero, 
        BinaryLogic::Zero 
    )));
    assert!(!res.main_failed());
    assert!(res.contains(&expected_log));
}

#[test] 
fn prediction_test_0_1() {
    let sys = System::new();
    sys.init_logger();
    let neuronal_network = Program::current(&sys);
    
    let mut expected_log = Log::builder()
        .dest(2)
        .payload(NNMessageOut::NeuronalNetworkCreated);
    let mut res = neuronal_network.send(2, NNMessageIn::SetLogicalXorNeuronalNetwork);
    assert!(!res.main_failed());
    assert!(res.contains(&expected_log));
    
    
    expected_log = Log::builder()
        .dest(2)
        .payload(NNMessageOut::Prediction(vec![String::from("0.9954791929")]));
    res = neuronal_network.send(2, NNMessageIn::PredictResultLogicalXor((
        BinaryLogic::Zero, 
        BinaryLogic::One, 
    )));
    assert!(!res.main_failed());
    assert!(res.contains(&expected_log));
}

#[test] 
fn prediction_test_1_0() {
    let sys = System::new();
    sys.init_logger();
    let neuronal_network = Program::current(&sys);
    
    let mut expected_log = Log::builder()
        .dest(2)
        .payload(NNMessageOut::NeuronalNetworkCreated);
    let mut res = neuronal_network.send(2, NNMessageIn::SetLogicalXorNeuronalNetwork);
    assert!(!res.main_failed());
    assert!(res.contains(&expected_log));
    
    
    expected_log = Log::builder()
        .dest(2)
        .payload(NNMessageOut::Prediction(vec![String::from("0.9954788196")]));
    res = neuronal_network.send(2, NNMessageIn::PredictResultLogicalXor((
        BinaryLogic::One, 
        BinaryLogic::Zero, 
    )));
    assert!(!res.main_failed());
    assert!(res.contains(&expected_log));
}

#[test] 
fn prediction_test_1_1() {
    let sys = System::new();
    sys.init_logger();
    let neuronal_network = Program::current(&sys);
    
    let mut expected_log = Log::builder()
        .dest(2)
        .payload(NNMessageOut::NeuronalNetworkCreated);
    let mut res = neuronal_network.send(2, NNMessageIn::SetLogicalXorNeuronalNetwork);
    assert!(!res.main_failed());
    assert!(res.contains(&expected_log));
    
    
    expected_log = Log::builder()
        .dest(2)
        .payload(NNMessageOut::Prediction(vec![String::from("0.046008109555")]));
    res = neuronal_network.send(2, NNMessageIn::PredictResultLogicalXor((
        BinaryLogic::One, 
        BinaryLogic::One, 
    )));
    assert!(!res.main_failed());
    assert!(res.contains(&expected_log));
}