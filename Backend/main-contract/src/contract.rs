use gstd::{prelude::*, msg, exec, collections::BTreeMap};
use main_contract_io::{
    InitMainContract,
    ContractAction,
    ContractEvent,
    SubscriptionType,
    SubscriberData,
    Period,
    Contract,
    State,
    ContractStateQuery,
    ContractStateReply,
    NeuronalNetworkId
};

static mut CONTRACT: Option<Contract> = None;

#[no_mangle]
extern "C" fn intit() {
    let config: InitMainContract = msg::load()
        .expect("Error loading init message");
    
    unsafe {
        CONTRACT = Some(Contract {
            owner: msg::source(),
            basic_plan_price: config.basic_plan_price,
            ultimate_plan_price: config.ultimate_plan_price,
            free_nn_ids: Vec::new(),
            basic_nn_ids: Vec::new(),
            ultimate_nn_ids: Vec::new(),
            nn_data: BTreeMap::new(),
            users_subscriptions: BTreeMap::new(),
        });
    };
}

#[gstd::async_main]
async fn main() {
    let message: ContractAction = msg::load()
        .expect("Error decoding 'ContractAction'");
    let contract = state_mut();
    
    
    match message {
        ContractAction::AddNeuronalNetwork(nn_data) => {
            // if msg::source() != contract.owner {
            //     msg::reply(ContractEvent::NotTheOwner, 0)
            //         .expect("Failed to send 'ContractEvent::NotTheOwner'");
            //     return;
            // }

            contract.free_nn_ids.push(nn_data.id);
            contract.nn_data.insert(nn_data.id, nn_data.clone());

            msg::reply(contract.add_nn_data(nn_data), 0)
                .expect("Failed to reply 'NeuronalNetworkAdded(NeuronalNetworkId)'");
        },
        ContractAction::AvailableNeuronalNetworks => {
            let user_id = msg::source();
            let user_suscription = contract.users_subscriptions.get(&user_id);
            if !user_suscription.is_some() {
                msg::reply(ContractEvent::UserIsNotRegistered(user_id), 0)
                    .expect("Failed to reply 'ContractEvent::UserIsNotRegistered(ActorId)'");
                return;
            }
            if user_suscription.is_none() {
                msg::reply(ContractEvent::UserIsNotRegistered(user_id), 0)
                    .expect("Failed to reply 'ContractEvent::SuscriptionExpired(SubscriptionType)'");
                return;
            }
            
            msg::reply(ContractEvent::NeuronalNetworksData(contract.nn_data_for_subscription(&user_suscription.unwrap().subscription_type)), 0)
                .expect("Failed to reply 'ContractEvent::NeuronalNetworksData(Data)'");
        },
        ContractAction::Subscribe(subscription_type) => {
            let period = Period::default();
            let user_id = msg::source();
            
            if contract.users_subscriptions.get(&user_id).is_some() { //.contains_key(&user_id) {
                msg::reply(ContractEvent::UserAlreadySubscribed(user_id), 0)
                    .expect("Failed to reply 'ContractEvent::UserAlreadySubscribed(ActorId)'");
                return;
            }
            
            match subscription_type {
                SubscriptionType::Free => {
                    let current_block = exec::block_height();
                    let current_date = exec::block_timestamp();
                    
                    contract.users_subscriptions.insert(
                        user_id, 
                        SubscriberData {
                            subscription_type: SubscriptionType::Free,
                            period: Period::NoPeriod,
                            subscription_start: Some((current_date, current_block)),
                            renewal_date: None,
                            expired: false
                        }
                    );
                    Contract::register_user_in_nns(&contract.free_nn_ids, &user_id).await;
                },
                SubscriptionType::Basic => {
                    let value = msg::value() as u64;
                    if value != contract.basic_plan_price {
                        msg::reply(ContractEvent::WrongFunds(contract.basic_plan_price), value as u128)
                            .expect("Failed to reply 'ContractEvent::WrongFunds(u64)'");
                        return;
                    }
                    
                    if msg::send_delayed(
                        exec::program_id(),
                        ContractAction::UpdateSubscription { subscriber: user_id },
                        0,
                        period.to_blocks(),
                    ).is_err() {
                        // Delayed message sending is needed for storage invalidation and update.
                        // If this "sanity" message sending was failed, then we consider subscriber
                        // as pending, so he can enable/withdraw his subscription receiving back
                        // money.
                        contract.users_subscriptions.insert(
                            user_id, 
                            SubscriberData {
                                subscription_type: SubscriptionType::Basic,
                                period,
                                subscription_start: None,
                                renewal_date: None,
                                expired: true
                            }
                        );
                        return;
                    }
                    
                    let start_block = exec::block_height();
                    let start_date = exec::block_timestamp();
                    
                    contract.users_subscriptions.insert(
                        user_id, 
                        SubscriberData {
                            subscription_type: SubscriptionType::Basic,
                            period,
                            subscription_start: Some((start_date, start_block)),
                            renewal_date: None,
                            expired: false
                        }
                    );
                    
                    Contract::register_user_in_nns(&contract.free_nn_ids, &user_id).await;
                    Contract::register_user_in_nns(&contract.basic_nn_ids, &user_id).await;
                },
                SubscriptionType::Ultimate => {
                    let value = msg::value() as u64;
                    
                    if value != contract.ultimate_plan_price {
                        msg::reply(ContractEvent::WrongFunds(contract.ultimate_plan_price), value as u128)
                            .expect("Failed to reply 'ContractEvent::WrongFunds(u64)'");
                        return;
                    }
                    
                    if msg::send_delayed(
                        exec::program_id(),
                        ContractAction::UpdateSubscription { subscriber: user_id },
                        0,
                        period.to_blocks(),
                    ).is_err() {
                        // Delayed message sending is needed for storage invalidation and update.
                        // If this "sanity" message sending was failed, then we consider subscriber
                        // as pending, so he can enable/withdraw his subscription receiving back
                        // money.
                        contract.users_subscriptions.insert(
                            user_id, 
                            SubscriberData {
                                subscription_type: SubscriptionType::Ultimate,
                                period,
                                subscription_start: None,
                                renewal_date: None,
                                expired: true
                            }
                        );
                        return;
                    }
                    
                    let start_block = exec::block_height();
                    let start_date = exec::block_timestamp();
                    
                    contract.users_subscriptions.insert(
                        user_id, 
                        SubscriberData {
                            subscription_type: SubscriptionType::Ultimate,
                            period,
                            subscription_start: Some((start_date, start_block)),
                            renewal_date: None,
                            expired: false
                        }
                    );
                    
                    Contract::register_user_in_nns(&contract.free_nn_ids, &user_id).await;
                    Contract::register_user_in_nns(&contract.basic_nn_ids, &user_id).await;
                    Contract::register_user_in_nns(&contract.ultimate_nn_ids, &user_id).await;
                }
            }
            
            msg::reply(ContractEvent::Subscribed, 0)
                .expect("Failed to reply with 'ContractEvent::Subscribed'");
        },
        ContractAction::UpdateSubscription { subscriber } => {
            let contract_id = exec::program_id();
            if msg::source() != contract_id {
                msg::reply(ContractEvent::NotTheOwner, 0)
                    .expect("Failed to reply 'ContractEvent::NotTheOwner'");
                return;
            }
            
            if !contract.users_subscriptions.contains_key(&subscriber) {
                msg::reply(ContractEvent::UserIsNotRegistered(subscriber), 0)
                    .expect("Failed to reply 'ContractEvent::UserIsNotRegistered(ActorId)'");
                return;
            }
            
            let mut subscription_type = SubscriptionType::default();
            contract.users_subscriptions
                .entry(subscriber)
                .and_modify(|user_data| {
                    subscription_type = user_data.subscription_type.clone();
                    *user_data = SubscriberData {
                        expired: true,
                        ..user_data.clone()
                    };
                });
            
            match subscription_type {
                SubscriptionType::Free => {
                    Contract::update_user_in_nns(&contract.free_nn_ids, &subscriber, true).await;
                },
                SubscriptionType::Basic => {
                    Contract::update_user_in_nns(&contract.free_nn_ids, &subscriber, true).await;
                    Contract::update_user_in_nns(&contract.basic_nn_ids, &subscriber, true).await;
                },
                SubscriptionType::Ultimate => {
                    Contract::update_user_in_nns(&contract.free_nn_ids, &subscriber, true).await;
                    Contract::update_user_in_nns(&contract.basic_nn_ids, &subscriber, true).await;
                    Contract::update_user_in_nns(&contract.ultimate_nn_ids, &subscriber, true).await;
                }
            }
        },
        ContractAction::CancelSubscription => {
            let subscriber = msg::source();
            
            if !contract.users_subscriptions.contains_key(&subscriber) {
                msg::reply(ContractEvent::UserIsNotRegistered(subscriber), 0)
                    .expect("Failed to reply 'ContractEvent::UserIsNotRegistered(ActorId)'");
                return;
            }
            
            contract.users_subscriptions
                .entry(subscriber)
                .and_modify(|user_data| {
                    *user_data = SubscriberData {
                        renewal_date: None,
                        expired: true,
                        ..user_data.clone()
                    };
                });
                
            msg::reply(ContractEvent::SubscriptionRenewed, 0)
                .expect("Failed to reply 'ContractEvent::SubscriptionRenewed'");
        },
        ContractAction::RenewSubscription => {
            let subscriber = msg::source();
            let subscription = contract.users_subscriptions.get(&subscriber);
            
            if subscription.is_none() {
                msg::reply(ContractEvent::UserIsNotRegistered(subscriber), 0)
                    .expect("Failed to reply 'ContractEvent::UserIsNotRegistered(ActorId)'");
                return;
            }
            let value = msg::value() as u64;            
            match subscription.unwrap().subscription_type {
                SubscriptionType::Basic => {
                    if contract.basic_plan_price != value {
                        msg::reply(ContractEvent::WrongFunds(contract.basic_plan_price), value as u128)
                            .expect("Failed to reply 'ContractEvent::WrongFunds(u64)'");
                        return;
                    }
                },
                SubscriptionType::Ultimate => {
                    if contract.ultimate_plan_price != value {
                        msg::reply(ContractEvent::WrongFunds(contract.ultimate_plan_price), value as u128)
                            .expect("Failed to reply 'ContractEvent::WrongFunds(u64)'");
                        return;
                    }
                }
                _ => {}
            }
            
            contract.users_subscriptions
                .entry(subscriber)
                .and_modify(|user_data| {
                    *user_data = SubscriberData {
                        renewal_date: None,
                        expired: true,
                        ..user_data.clone()
                    };
                });
            
            msg::reply(ContractEvent::SubscriptionRenewed, 0)
                .expect("Failed to reply 'ContractEvent::SubscriptionRenewed'");
        }
    }
}

#[no_mangle]
extern "C" fn state() {
    let message = msg::load()
        .expect("Error loading message");
    let contract = state_ref();

    match message {
        ContractStateQuery::UserHasSubscription(user_id) => {
            let has_subscription = contract.users_subscriptions.contains_key(&user_id);

            msg::reply(ContractStateReply::UserHasSubscription(has_subscription), 0)
                .expect("Error sending reply");
        },
        ContractStateQuery::UserSubscriptionType(user_id) => {
            let user_subscription_data = contract.users_subscriptions.get(&user_id);

            if let Some(subscription_data) = user_subscription_data {
                msg::reply(ContractStateReply::UserSubscriptionType(subscription_data.subscription_type.clone()), 0)
                    .expect("Error sending reply state");
            } else {
                msg::reply(ContractStateReply::UserIsNotSubscribed, 0)
                    .expect("Error sending reply state");
            }
        },
        ContractStateQuery::FreeNNAdresses(user_id) => {
            if  !contract.users_subscriptions.contains_key(&user_id) && contract.owner != user_id {
                msg::reply(ContractStateReply::UserIsNotSubscribed, 0)
                    .expect("Error sending reply state");
                return;
            }

            let free_nn_addresses = contract.free_nn_ids.clone();

            msg::reply(ContractStateReply::FreeNNAdresses(free_nn_addresses), 0)
                    .expect("Error sending reply state");
        },
        ContractStateQuery::BasicNNAdresses(user_id) => {
            if  !contract.users_subscriptions.contains_key(&user_id) && contract.owner != user_id {
                msg::reply(ContractStateReply::UserIsNotSubscribed, 0)
                    .expect("Error sending reply state");
                return;
            }

            let basic_nn_address = contract.basic_nn_ids.clone();

            msg::reply(ContractStateReply::BasicNNAdresses(basic_nn_address), 0)
                    .expect("Error sending reply state");
        },
        ContractStateQuery::UltimateNNAddresses(user_id) => {
            if  !contract.users_subscriptions.contains_key(&user_id) && contract.owner != user_id {
                msg::reply(ContractStateReply::UserIsNotSubscribed, 0)
                    .expect("Error sending reply state");
                return;
            }

            let ultimate_nn_address = contract.ultimate_nn_ids.clone();

            msg::reply(ContractStateReply::UltimateNNAddresses(ultimate_nn_address), 0)
                    .expect("Error sending reply state");
        },
        ContractStateQuery::NeuralNetworkData {
            nn_address,
            user_id
        } => {
            if  !contract.users_subscriptions.contains_key(&user_id) && contract.owner != user_id {
                msg::reply(ContractStateReply::UserIsNotSubscribed, 0)
                    .expect("Error sending reply state");
                return;
            }

            let Some(contract_data) = contract.nn_data.get(&nn_address) else {
                msg::reply(ContractStateReply::NeuralNetworkAddresDoesNotExists, 0)
                    .expect("Error sending reply state");
                return;
            };

            msg::reply(ContractStateReply::NeuralNetworkData(contract_data.clone()), 0)
                    .expect("Error sending reply state");
        }
        ContractStateQuery::All => {
            let state: State = contract.clone().into();

            msg::reply(ContractStateReply::All(state), 0)
                    .expect("Error sending reply state");
        }
    }
}

fn state_mut() -> &'static mut Contract {
    let state = unsafe { CONTRACT.as_mut() };
    debug_assert!(state.is_some(), "State isn't initialized");
    unsafe { state.unwrap_unchecked() }
}

fn state_ref() -> &'static Contract {
    let state = unsafe { CONTRACT.as_ref() };
    debug_assert!(state.is_some(), "State isn't initialized");
    unsafe { state.unwrap_unchecked() }
}
