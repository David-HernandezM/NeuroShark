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
fn accept_new_user() {
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
}

#[test]
fn accept_new_fail() {
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
        .dest(ACCOUNTS[2])
        .payload(LogicGateEvent::UserIsNotTheOwner);
        
    let mut res = xgnn.send(
        ACCOUNTS[2],
         LogicGateAction::AcceptUser(ACCOUNTS[1].into())
    );
    
    assert!(!res.main_failed());
    assert!(res.contains(&expected_log));
}

#[test]
fn user_suscripcion_expired() {
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
}


#[test]
fn user_suscripcion_expired_fail() {
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
        .dest(ACCOUNTS[2])
        .payload(LogicGateEvent::UserIsNotTheOwner);
        
    let mut res = xgnn.send(
        ACCOUNTS[2],
        LogicGateAction::UserSubscriptionExpired(ACCOUNTS[1].into())
    );
    
    assert!(!res.main_failed());
    assert!(res.contains(&expected_log));
}



#[test]
fn user_suscripcion_renewed() {
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
        LogicGateAction::UserSubscriptionRenewed(ACCOUNTS[1].into())
    );
    
    assert!(!res.main_failed());
    assert!(res.contains(&expected_log));
}


#[test]
fn user_suscripcion_renewed_fail() {
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
        .dest(ACCOUNTS[2])
        .payload(LogicGateEvent::UserIsNotTheOwner);
        
    let mut res = xgnn.send(
        ACCOUNTS[2],
        LogicGateAction::UserSubscriptionRenewed(ACCOUNTS[1].into())
    );
    
    assert!(!res.main_failed());
    assert!(res.contains(&expected_log));
}