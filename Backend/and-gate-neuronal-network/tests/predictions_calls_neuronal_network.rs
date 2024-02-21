use gtest::{ Log, System, Program };
use program_io::{
    LogicGateAction,
    LogicGateEvent,
    BinaryLogic,
    LogicGate,
    Predict,
    State
};

const ACCOUNTS: &[u64] = &[3, 4, 5];

#[test] 
fn prediction_not_subscribed() {
    let sys = System::new();
    sys.init_logger();
    let xgnn = Program::current(&sys);
    
    let expected_log = Log::builder()
        .dest(ACCOUNTS[0])
        .payload(LogicGateEvent::NeuronalNetworkCreated);
        
    let mut res = xgnn.send(
        ACCOUNTS[0],
        String::from("")
    );
    
    assert!(!res.main_failed());
    assert!(res.contains(&expected_log));
    
    let expected_log = Log::builder()
        .dest(ACCOUNTS[1])
        .payload(LogicGateEvent::UserIsNotSubscribed);
    res = xgnn.send(ACCOUNTS[1], LogicGateAction::Predict((
        BinaryLogic::Zero, 
        BinaryLogic::Zero 
    )));
    assert!(!res.main_failed());
    assert!(res.contains(&expected_log));
}

#[test]
fn prediction_suscribed_expired() {
    let sys = System::new();
    sys.init_logger();
    let xgnn = Program::current(&sys);
    
    let expected_log = Log::builder()
        .dest(ACCOUNTS[0])
        .payload(LogicGateEvent::NeuronalNetworkCreated);
        
    let mut res = xgnn.send(
        ACCOUNTS[0],
        String::from("")
    );
    
    assert!(!res.main_failed());
    assert!(res.contains(&expected_log));
    
    let expected_log = Log::builder()
        .dest(ACCOUNTS[0])
        .payload(LogicGateEvent::UserAccepted);
        
    let mut res = xgnn.send(
        ACCOUNTS[0],
         LogicGateAction::AcceptUser(ACCOUNTS[1].into())
    );
    
    assert!(!res.main_failed());
    assert!(res.contains(&expected_log));
    
    let expected_log = Log::builder()
        .dest(ACCOUNTS[0])
        .payload(LogicGateEvent::SubscriptionUpdated);
        
    let mut res = xgnn.send(
        ACCOUNTS[0],
        LogicGateAction::UserSubscriptionExpired(ACCOUNTS[1].into())
    );
    
    assert!(!res.main_failed());
    assert!(res.contains(&expected_log));
    
    let expected_log = Log::builder()
        .dest(ACCOUNTS[1])
        .payload(LogicGateEvent::SubscriptionExpired);
    res = xgnn.send(ACCOUNTS[1], LogicGateAction::Predict((
        BinaryLogic::Zero, 
        BinaryLogic::Zero 
    )));
    assert!(!res.main_failed());
    assert!(res.contains(&expected_log));
}

#[test] 
fn prediction_test_0_0() {
    let sys = System::new();
    sys.init_logger();
    let xgnn = Program::current(&sys);
    
    let expected_log = Log::builder()
        .dest(ACCOUNTS[0])
        .payload(LogicGateEvent::NeuronalNetworkCreated);
        
    let mut res = xgnn.send(
        ACCOUNTS[0],
        String::from("")
    );
    
    assert!(!res.main_failed());
    assert!(res.contains(&expected_log));
    
    let expected_log = Log::builder()
        .dest(ACCOUNTS[0])
        .payload(LogicGateEvent::UserAccepted);
        
    let mut res = xgnn.send(
        ACCOUNTS[0],
        LogicGateAction::AcceptUser(ACCOUNTS[1].into())
    );
    
    assert!(!res.main_failed());
    assert!(res.contains(&expected_log));
    
    let expected_log = Log::builder()
        .dest(ACCOUNTS[1])
        .payload(LogicGateEvent::Prediction(vec![String::from("0.002476546847")]));
    res = xgnn.send(ACCOUNTS[1], LogicGateAction::Predict((
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
    let xgnn = Program::current(&sys);
    
    let expected_log = Log::builder()
        .dest(ACCOUNTS[0])
        .payload(LogicGateEvent::NeuronalNetworkCreated);
        
    let mut res = xgnn.send(
        ACCOUNTS[0],
        String::from("")
    );
    
    assert!(!res.main_failed());
    assert!(res.contains(&expected_log));
    
    let expected_log = Log::builder()
        .dest(ACCOUNTS[0])
        .payload(LogicGateEvent::UserAccepted);
        
    let mut res = xgnn.send(
        ACCOUNTS[0],
        LogicGateAction::AcceptUser(ACCOUNTS[1].into())
    );
    
    assert!(!res.main_failed());
    assert!(res.contains(&expected_log));
    
    let expected_log = Log::builder()
        .dest(ACCOUNTS[1])
        .payload(LogicGateEvent::Prediction(vec![String::from("0.99409868215")]));
    res = xgnn.send(ACCOUNTS[1],  LogicGateAction::Predict((
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
    let xgnn = Program::current(&sys);
    
    let expected_log = Log::builder()
        .dest(ACCOUNTS[0])
        .payload(LogicGateEvent::NeuronalNetworkCreated);
        
    let mut res = xgnn.send(
        ACCOUNTS[0],
        String::from("")
    );
    
    assert!(!res.main_failed());
    assert!(res.contains(&expected_log));
    
    let expected_log = Log::builder()
        .dest(ACCOUNTS[0])
        .payload(LogicGateEvent::UserAccepted);
        
    let mut res = xgnn.send(
        ACCOUNTS[0],
        LogicGateAction::AcceptUser(ACCOUNTS[1].into())
    );
    
    assert!(!res.main_failed());
    assert!(res.contains(&expected_log));
    
    let expected_log = Log::builder()
        .dest(ACCOUNTS[1])
        .payload(LogicGateEvent::Prediction(vec![String::from("0.98574961043")]));
    res = xgnn.send(ACCOUNTS[1],  LogicGateAction::Predict((
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
    let xgnn = Program::current(&sys);
    
    let expected_log = Log::builder()
        .dest(ACCOUNTS[0])
        .payload(LogicGateEvent::NeuronalNetworkCreated);
        
    let mut res = xgnn.send(
        ACCOUNTS[0],
        String::from("")
    );
    
    assert!(!res.main_failed());
    assert!(res.contains(&expected_log));
    
    let expected_log = Log::builder()
        .dest(ACCOUNTS[0])
        .payload(LogicGateEvent::UserAccepted);
        
    let mut res = xgnn.send(
        ACCOUNTS[0],
        LogicGateAction::AcceptUser(ACCOUNTS[1].into())
    );
    
    assert!(!res.main_failed());
    assert!(res.contains(&expected_log));
    
    
    let expected_log = Log::builder()
        .dest(ACCOUNTS[1])
        .payload(LogicGateEvent::Prediction(vec![String::from("0.020051109851")]));
    res = xgnn.send(ACCOUNTS[1],  LogicGateAction::Predict((
        BinaryLogic::One, 
        BinaryLogic::One, 
    )));
    assert!(!res.main_failed());
    assert!(res.contains(&expected_log));
}