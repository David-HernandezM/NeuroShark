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
fn init_xor_neuronal_network() {
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
}