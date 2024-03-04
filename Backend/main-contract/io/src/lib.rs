#![no_std]
use gstd::{prelude::*, Vec, collections::BTreeMap, ActorId, msg};
use gmeta::{Metadata, In, Out, InOut};

pub struct ProgramMetadata;

pub type NeuronalNetworkId = ActorId;
pub type UserId = ActorId;
pub type Expired = bool;

impl Metadata for ProgramMetadata {
    type Init = In<InitMainContract>;
    type Handle = InOut<ContractAction, ContractEvent>;
    type Others = InOut<ContractAction, ContractEvent>;
    type Reply = ();
    type Signal = ();
    type State = InOut<ContractStateQuery, ContractStateReply>;
}

#[derive(Clone)]
pub struct Contract {
    pub owner: ActorId,
    pub basic_plan_price: u64,
    pub ultimate_plan_price: u64,
    pub free_nn_ids: Vec<NeuronalNetworkId>,
    pub basic_nn_ids: Vec<NeuronalNetworkId>,
    pub ultimate_nn_ids: Vec<NeuronalNetworkId>,
    pub nn_data: BTreeMap<NeuronalNetworkId, NeuronalNetworkData>,
    pub users_subscriptions: BTreeMap<UserId, SubscriberData>,
}

impl Contract {
    pub fn add_nn_data(&mut self, nn_data: NeuronalNetworkData) -> ContractEvent {
        let nn_id = nn_data.id.clone();
        match nn_data.subscription_type.clone() {
            SubscriptionType::Free => self.free_nn_ids.push(nn_id),
            SubscriptionType::Basic => self.basic_nn_ids.push(nn_id),
            SubscriptionType::Ultimate => self.ultimate_nn_ids.push(nn_id)
        }
        self.nn_data.insert(nn_id, nn_data);
        
        ContractEvent::NeuronalNetworkAdded(nn_id)
    }
    
    pub fn nn_data_for_subscription(&self, subscription: &SubscriptionType) -> Vec<NeuronalNetworkData> {
        let mut nn_data: Vec<NeuronalNetworkData> = vec![];
        match subscription {
            SubscriptionType::Free => {
                nn_data = self.free_nn_data();
            },
            SubscriptionType::Basic => {
                nn_data = self.free_nn_data();
                nn_data.extend(self.basic_nn_data());
            },
            SubscriptionType::Ultimate => {
                nn_data = self.free_nn_data();
                nn_data.extend(self.basic_nn_data());
                nn_data.extend(self.ultimate_nn_data());
            }
        }
        
        nn_data
    }
    
    pub fn free_nn_data(&self) -> Vec<NeuronalNetworkData> {
        let mut data: Vec<NeuronalNetworkData> = vec![];        
        self.free_nn_ids
            .iter()
            .for_each(|nn_id| {
                if let Some(nn_data) = self.nn_data.get(nn_id) {
                    data.push(nn_data.clone());
                }
            });
        data
    }
    
    pub fn basic_nn_data(&self) -> Vec<NeuronalNetworkData> {
        let mut data: Vec<NeuronalNetworkData> = vec![];        
        self.basic_nn_ids
            .iter()
            .for_each(|nn_id| {
                if let Some(nn_data) = self.nn_data.get(nn_id) {
                    data.push(nn_data.clone());
                }
            });
        data
    }
    
    pub fn ultimate_nn_data(&self) -> Vec<NeuronalNetworkData> {
        let mut data: Vec<NeuronalNetworkData> = vec![];        
        self.ultimate_nn_ids
            .iter()
            .for_each(|nn_id| {
                if let Some(nn_data) = self.nn_data.get(nn_id) {
                    data.push(nn_data.clone());
                }
            });
        data
    }
    
    pub fn get_nn_data_by_id(&self, nn_id: NeuronalNetworkId) -> Result<NeuronalNetworkData, ()> {
        let nn_data = self.nn_data.get(&nn_id);
        if !nn_data.is_some() {
            return Err(());
        }
        Ok(nn_data.unwrap().clone())
    }
    
    pub async fn register_user_in_nns(nn_ids: &Vec<NeuronalNetworkId>, user_id: &ActorId) {
        for nn_id in nn_ids.iter() {
            msg::send_for_reply_as::<XGNNAction, XGNNEvent>(
                    *nn_id, 
                    XGNNAction::AcceptUser(user_id.clone()), 
                    0, 
                    0
                )
                .expect("Error during a sending message to a Neuroral Network program")
                .await
                .expect("Unable to decode XGNNEvent");
        }
    }
    
    pub async fn update_user_in_nns(nn_ids: &Vec<NeuronalNetworkId>, user_id: &ActorId, expired: Expired) {
        for nn_id in nn_ids.iter() {
            let message = if expired {
                    XGNNAction::UserSubscriptionExpired(user_id.clone())
                } else {
                    XGNNAction::UserSubscriptionRenewed(user_id.clone())
                };
                msg::send_for_reply_as::<XGNNAction, XGNNEvent>(
                    *nn_id, 
                    message, 
                    0, 
                    0
                )
                .expect("Error during a sending message to a Neuroral Network program")
                .await
                .expect("Unable to decode XGNNEvent");
        }
    }
}

#[derive(Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum BinaryLogic {
    One,
    Zero
}

#[derive(Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum XGNNAction {
    AcceptUser(ActorId),
    UserSubscriptionExpired(UserId),
    UserSubscriptionRenewed(UserId),
    Predict((BinaryLogic, BinaryLogic)),
}

#[derive(Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum XGNNEvent {
    NeuronalNetworkCreated,
    UserIsNotTheOwner,
    UserIsNotSubscribed,
    SubscriptionUpdated,
    SubscriptionExpired,
    UserAccepted,
    Prediction(Vec<String>)   
}

#[derive(Default, Debug, Encode, Decode, PartialEq, Eq, PartialOrd, Ord, Clone, TypeInfo, Hash)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct InitMainContract {
    pub basic_plan_price: u64,
    pub ultimate_plan_price: u64,
}

#[derive(Default, Debug, Encode, Decode, PartialEq, Eq, PartialOrd, Ord, Clone, TypeInfo, Hash)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum SubscriptionType {
    #[default]
    Free,
    Basic,
    Ultimate
}

#[derive(Debug, Encode, Decode, PartialEq, Eq, PartialOrd, Ord, Clone, TypeInfo, Hash)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum ContractAction {
    AddNeuronalNetwork(NeuronalNetworkData),
    AvailableNeuronalNetworks,
    Subscribe(SubscriptionType),
    UpdateSubscription { subscriber: ActorId },
    CancelSubscription,
    RenewSubscription
}

#[derive(Debug, Encode, Decode, PartialEq, Eq, PartialOrd, Ord, Clone, TypeInfo, Hash)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum ContractEvent {
    UserIsNotRegistered(UserId),
    UserAlreadySubscribed(ActorId),
    NeuronalNetworkAdded(NeuronalNetworkId),
    NeuronalNetworksData(Vec<NeuronalNetworkData>),
    SubscriptionCanceled,
    SubscriptionChanged,
    SubscriptionRenewed,
    SuscriptionExpired(SubscriptionType),
    NotEnoughFunds,
    NoMainContract,
    NotTheOwner,
    Subscribed,
    WrongFunds(u64)
}

#[derive(Debug, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum ContractStateQuery {
    UserHasSubscription(ActorId),
    UserSubscriptionType(ActorId),
    FreeNNAdresses(ActorId),
    BasicNNAdresses(ActorId),
    UltimateNNAddresses(ActorId),
    NeuralNetworkData {
        nn_address: NeuronalNetworkId,
        user_id: ActorId
    },
    All
}

#[derive(Debug, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum ContractStateReply {
    UserHasSubscription(bool),
    UserSubscriptionType(SubscriptionType),
    FreeNNAdresses(Vec<NeuronalNetworkId>),
    BasicNNAdresses(Vec<NeuronalNetworkId>),
    UltimateNNAddresses(Vec<NeuronalNetworkId>),
    NeuralNetworkData(NeuronalNetworkData),
    UserIsNotSubscribed,
    NeuralNetworkAddresDoesNotExists,
    All(State)
}

#[derive(Default, Debug, Encode, Decode, PartialEq, Eq, PartialOrd, Ord, Clone, TypeInfo, Hash)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct NeuronalNetworkData {
    pub id: ActorId,
    pub description: String,
    pub subscription_type: SubscriptionType,
    // Base64 coded svg image
    pub image: String
}

#[derive(Default, Debug, Encode, Decode, Clone, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct State {
    owner: ActorId,
    basic_plan_price: u64,
    ultimate_plan_price: u64,
    free_nn_ids: Vec<NeuronalNetworkId>,
    basic_nn_ids: Vec<NeuronalNetworkId>,
    ultimate_nn_ids: Vec<NeuronalNetworkId>,
    nn_data: Vec<(NeuronalNetworkId, NeuronalNetworkData)>,
    users_subscriptions: Vec<(ActorId, SubscriberData)>
}

#[derive(Debug, Clone, Copy, Default, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum Period {
    NoPeriod,
    Year,
    NineMonths,
    SixMonths,
    ThreeMonths,
    #[default]
    Month,
}

/// Subscriber's data
#[derive(Debug, Clone, Default, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct SubscriberData {
    pub subscription_type: SubscriptionType,
    /// Subscription period
    pub period: Period,
    // TODO [optimization] this must be calculated off-chain
    /// Subscription start timestamp and block number.
    ///
    /// If `None`, means that subscriber has paid for the
    /// subscription, but didn't succeed sending delayed
    /// message for subscription check/renewal.
    pub subscription_start: Option<(u64, u32)>,
    // TODO [optimization] this must be calculated off-chain
    /// Subscription renewal date.
    ///
    /// If None, then no renewal desired.
    pub renewal_date: Option<(u64, u32)>,
    pub expired: bool
}

/// Subscriber's state
#[derive(Debug, Clone, Default, Encode, Decode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct SubscriberDataState {
    pub is_active: bool,
    pub start_date: u64,
    pub start_block: u32,
    pub end_date: u64,
    pub end_block: u32,
    pub period: Period,
    pub will_renew: bool,
    pub price: u128,
}


impl Period {
    // TODO [cleanness] Must be changeable
    const TARGET_BLOCK_TIME: u32 = Self::SECOND;
    const SECOND: u32 = 1;

    pub fn minimal_unit() -> Self {
        Self::Month
    }

    pub fn as_units(&self) -> u128 {
        match self {
            Period::NoPeriod => 0,
            Period::Year => 12,
            Period::NineMonths => 9,
            Period::SixMonths => 6,
            Period::ThreeMonths => 3,
            Period::Month => 1,
        }
    }

    pub fn to_blocks(&self) -> u32 {
        self.as_secs() / Self::TARGET_BLOCK_TIME
    }

    pub fn as_millis(&self) -> u64 {
        self.as_secs() as u64 * 1000
    }

    fn as_secs(&self) -> u32 {
        match self {
            Period::NoPeriod => 0,
            Period::Year => Self::Month.as_secs() * 12,
            Period::NineMonths => Self::Month.as_secs() * 9,
            Period::SixMonths => Self::Month.as_secs() * 6,
            Period::ThreeMonths => Self::Month.as_secs() * 3,
            Period::Month => Self::SECOND * 30,
        }
    }
}



impl From<Contract> for State {
    fn from(value: Contract) -> Self {
        let Contract {
            owner,
            basic_plan_price,
            ultimate_plan_price,
            free_nn_ids,
            basic_nn_ids,
            ultimate_nn_ids,
            nn_data,
            users_subscriptions
        } = value;

        let users_subscriptions = users_subscriptions
            .iter()
            .map(|(user, subscription)| (*user, subscription.clone()))
            .collect();

        let nn_data = nn_data
            .iter()
            .map(|(nn_id, nn_data)| (*nn_id, nn_data.clone()))
            .collect();

        Self {
            owner,
            basic_plan_price,
            free_nn_ids,
            ultimate_plan_price,
            basic_nn_ids,
            ultimate_nn_ids,
            nn_data,
            users_subscriptions
        }
    }
} 