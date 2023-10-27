use gtest::{ Log, System, Program };
use program_io::{
    InitMainContract,
    ContractAction,
    ContractEvent,
    NeuronalNetworkData,
    SubscriptionType
};

const USERS: &[u64] = &[3, 4, 5];
const NN_IDS: &[u64] = &[11, 12, 13];
const BASIC_PLAN: u64 = 600;
const ULTIMATE_PLAN: u64 = 700;


#[test]
fn subscribe_user_basic() {
    let sys = System::new();
    sys.init_logger();
    let main_contract = Program::current(&sys);
    
    let mut res = main_contract.send(
        USERS[0],
        InitMainContract {
            basic_plan_price: BASIC_PLAN,
            ultimate_plan_price: ULTIMATE_PLAN
        }
    );
    
    assert!(!res.main_failed());
    
    // Adding new Neuronals Networks 
    
    let mut expected_log = Log::builder()
        .dest(USERS[0])
        .payload(ContractEvent::NeuronalNetworkAdded(USERS[1].into()));
        
    res = main_contract.send(
        USERS[0],
        ContractAction::AddNeuronalNetwork(free_neuronal_network())
    );
    
    assert!(!res.main_failed());
    assert!(res.contains(&expected_log));
    
    expected_log = Log::builder()
        .dest(USERS[0])
        .payload(ContractEvent::NeuronalNetworkAdded(USERS[1].into()));
        
    res = main_contract.send(
        USERS[0],
        ContractAction::AddNeuronalNetwork(basic_neuronal_network())
    );
    
    assert!(!res.main_failed());
    assert!(res.contains(&expected_log));
    
    // Subscribing user to basic neuronal networks
    
    sys.mint_to(USERS[2], (BASIC_PLAN * 200).into());

    expected_log = Log::builder()
        .dest(USERS[2])
        .payload(ContractEvent::Subscribed);
        
    res = main_contract.send_with_value(
        USERS[2],
        ContractAction::Subscribe(SubscriptionType::Basic),
        BASIC_PLAN.into()
    );
    
    assert!(!res.main_failed());
    assert!(res.contains(&expected_log));
}

#[test]
fn subscribe_user_basic_fail() {
    let sys = System::new();
    sys.init_logger();
    let main_contract = Program::current(&sys);
    
    let mut res = main_contract.send(
        USERS[0],
        InitMainContract {
            basic_plan_price: BASIC_PLAN,
            ultimate_plan_price: ULTIMATE_PLAN
        }
    );
    
    assert!(!res.main_failed());
    
    // Adding new Neuronal Network 
    
    let mut expected_log = Log::builder()
        .dest(USERS[0])
        .payload(ContractEvent::NeuronalNetworkAdded(USERS[1].into()));
        
    res = main_contract.send(
        USERS[0],
        ContractAction::AddNeuronalNetwork(free_neuronal_network())
    );
    
    assert!(!res.main_failed());
    assert!(res.contains(&expected_log));
    
    expected_log = Log::builder()
        .dest(USERS[0])
        .payload(ContractEvent::NeuronalNetworkAdded(USERS[1].into()));
        
    res = main_contract.send(
        USERS[0],
        ContractAction::AddNeuronalNetwork(basic_neuronal_network())
    );
    
    assert!(!res.main_failed());
    assert!(res.contains(&expected_log));
    
    // must fail due to insufficient funds 
    
    sys.mint_to(USERS[2], (BASIC_PLAN * 200).into());

    expected_log = Log::builder()
        .dest(USERS[2])
        .payload(ContractEvent::WrongFunds(BASIC_PLAN));
        
    res = main_contract.send_with_value(
        USERS[2],
        ContractAction::Subscribe(SubscriptionType::Basic),
        (BASIC_PLAN - 10).into()
    );
    
    assert!(!res.main_failed());
    assert!(res.contains(&expected_log));
    
    // must fail due to excesive funds 
    
    expected_log = Log::builder()
        .dest(USERS[2])
        .payload(ContractEvent::WrongFunds(BASIC_PLAN));
        
    res = main_contract.send_with_value(
        USERS[2],
        ContractAction::Subscribe(SubscriptionType::Basic),
        (BASIC_PLAN + 10).into()
    );
    
    assert!(!res.main_failed());
    assert!(res.contains(&expected_log));
    
    // correct subscription
    
    expected_log = Log::builder()
        .dest(USERS[2])
        .payload(ContractEvent::Subscribed);
        
    res = main_contract.send_with_value(
        USERS[2],
        ContractAction::Subscribe(SubscriptionType::Basic),
        BASIC_PLAN.into()
    );
    
    assert!(!res.main_failed());
    assert!(res.contains(&expected_log));
    
    // must fail since the user is already subscribed
    
    expected_log = Log::builder()
        .dest(USERS[2])
        .payload(ContractEvent::UserAlreadySubscribed(USERS[2].into()));
        
    res = main_contract.send_with_value(
        USERS[2],
        ContractAction::Subscribe(SubscriptionType::Basic),
        BASIC_PLAN.into()
    );
    
    assert!(!res.main_failed());
    assert!(res.contains(&expected_log));
}



#[test]
fn subscribe_user_ultimate() {
    let sys = System::new();
    sys.init_logger();
    let main_contract = Program::current(&sys);
    
    let mut res = main_contract.send(
        USERS[0],
        InitMainContract {
            basic_plan_price: BASIC_PLAN,
            ultimate_plan_price: ULTIMATE_PLAN
        }
    );
    
    assert!(!res.main_failed());
    
    // Adding new Neuronal Network 
    
    let mut expected_log = Log::builder()
        .dest(USERS[0])
        .payload(ContractEvent::NeuronalNetworkAdded(USERS[1].into()));
        
    res = main_contract.send(
        USERS[0],
        ContractAction::AddNeuronalNetwork(free_neuronal_network())
    );
    
    assert!(!res.main_failed());
    assert!(res.contains(&expected_log));
    
    expected_log = Log::builder()
        .dest(USERS[0])
        .payload(ContractEvent::NeuronalNetworkAdded(USERS[1].into()));
        
    res = main_contract.send(
        USERS[0],
        ContractAction::AddNeuronalNetwork(basic_neuronal_network())
    );
    
    assert!(!res.main_failed());
    assert!(res.contains(&expected_log));
    
    expected_log = Log::builder()
        .dest(USERS[0])
        .payload(ContractEvent::NeuronalNetworkAdded(USERS[1].into()));
        
    res = main_contract.send(
        USERS[0],
        ContractAction::AddNeuronalNetwork(ultimate_neuronal_network())
    );
    
    assert!(!res.main_failed());
    assert!(res.contains(&expected_log));
    
    // Subscribing user to basic neuronal networks
    
    sys.mint_to(USERS[2], (ULTIMATE_PLAN * 200).into());

    expected_log = Log::builder()
        .dest(USERS[2])
        .payload(ContractEvent::Subscribed);
        
    res = main_contract.send_with_value(
        USERS[2],
        ContractAction::Subscribe(SubscriptionType::Ultimate),
        ULTIMATE_PLAN.into()
    );
    
    assert!(!res.main_failed());
    assert!(res.contains(&expected_log));
}

#[test]
fn subscribe_user_ultimate_fail() {
    let sys = System::new();
    sys.init_logger();
    let main_contract = Program::current(&sys);
    
    let mut res = main_contract.send(
        USERS[0],
        InitMainContract {
            basic_plan_price: BASIC_PLAN,
            ultimate_plan_price: ULTIMATE_PLAN
        }
    );
    
    assert!(!res.main_failed());
    
    // Adding new Neuronal Network 
    
    let mut expected_log = Log::builder()
        .dest(USERS[0])
        .payload(ContractEvent::NeuronalNetworkAdded(USERS[1].into()));
        
    res = main_contract.send(
        USERS[0],
        ContractAction::AddNeuronalNetwork(free_neuronal_network())
    );
    
    assert!(!res.main_failed());
    assert!(res.contains(&expected_log));
    
    expected_log = Log::builder()
        .dest(USERS[0])
        .payload(ContractEvent::NeuronalNetworkAdded(USERS[1].into()));
        
    res = main_contract.send(
        USERS[0],
        ContractAction::AddNeuronalNetwork(basic_neuronal_network())
    );
    
    assert!(!res.main_failed());
    assert!(res.contains(&expected_log));
    
    expected_log = Log::builder()
        .dest(USERS[0])
        .payload(ContractEvent::NeuronalNetworkAdded(USERS[1].into()));
        
    res = main_contract.send(
        USERS[0],
        ContractAction::AddNeuronalNetwork(ultimate_neuronal_network())
    );
    
    assert!(!res.main_failed());
    assert!(res.contains(&expected_log));
    
    // must fail due to insufficient funds 
    
    sys.mint_to(USERS[2], (BASIC_PLAN * 200).into());

    expected_log = Log::builder()
        .dest(USERS[2])
        .payload(ContractEvent::WrongFunds(ULTIMATE_PLAN));
        
    res = main_contract.send_with_value(
        USERS[2],
        ContractAction::Subscribe(SubscriptionType::Ultimate),
        (ULTIMATE_PLAN - 10).into()
    );
    
    assert!(!res.main_failed());
    assert!(res.contains(&expected_log));
    
    // must fail due to excesive funds 
    
    expected_log = Log::builder()
        .dest(USERS[2])
        .payload(ContractEvent::WrongFunds(ULTIMATE_PLAN));
        
    res = main_contract.send_with_value(
        USERS[2],
        ContractAction::Subscribe(SubscriptionType::Ultimate),
        (ULTIMATE_PLAN + 10).into()
    );
    
    assert!(!res.main_failed());
    assert!(res.contains(&expected_log));
    
    // correct subscription
    
    expected_log = Log::builder()
        .dest(USERS[2])
        .payload(ContractEvent::Subscribed);
        
    res = main_contract.send_with_value(
        USERS[2],
        ContractAction::Subscribe(SubscriptionType::Ultimate),
        ULTIMATE_PLAN.into()
    );
    
    assert!(!res.main_failed());
    assert!(res.contains(&expected_log));
    
    // must fail since the user is already subscribed
    
    expected_log = Log::builder()
        .dest(USERS[2])
        .payload(ContractEvent::UserAlreadySubscribed(USERS[2].into()));
        
    res = main_contract.send_with_value(
        USERS[2],
        ContractAction::Subscribe(SubscriptionType::Ultimate),
        ULTIMATE_PLAN.into()
    );
    
    assert!(!res.main_failed());
    assert!(res.contains(&expected_log));
}


fn free_neuronal_network() -> NeuronalNetworkData {
    NeuronalNetworkData {
        id: NN_IDS[1].into(),
        description: String::from("And Gate Neuronal Network"),
        subscription_type: SubscriptionType::Free,
        image: String::from("base64Image:%w32dskfdflldsf")
    }
}

fn basic_neuronal_network() -> NeuronalNetworkData {
    NeuronalNetworkData {
        id: NN_IDS[1].into(),
        description: String::from("Xor Gate Neuronal Network"),
        subscription_type: SubscriptionType::Basic,
        image: String::from("base64Image:%asdjfadfi923k.-2323dfssa")
    }
}

fn ultimate_neuronal_network() -> NeuronalNetworkData {
    NeuronalNetworkData {
        id: NN_IDS[1].into(),
        description: String::from("Nand Gate Neuronal Network"),
        subscription_type: SubscriptionType::Ultimate,
        image: String::from("base64Image:%w32dskfdsdfasdfadsf23r232flldsf")
    }
}

