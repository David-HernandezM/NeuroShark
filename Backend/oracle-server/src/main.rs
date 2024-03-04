use gear_core::ids::{
    MessageId,
    ProgramId
};

use gclient::{
    GearApi,
    WSAddress,
    EventListener,
    EventProcessor
};

use gstd::{Encode, CodeId, ActorId, TypeInfo};
use hex_literal;
use hex_fmt::HexFmt;

use doodles_prediction_neuronal_network_io::{DoodleNNAction, fraction::Fraction};

const DIVISOR: i128 = 1000000000000;
const MAX_NUM: i128 = 9999999999999;

#[tokio::main]
async fn main() {
    const MNEMONIC: &str = "strong orchard plastic arena pyramid lobster lonely rich stomach label clog rubber";
    const MAIN_CONTRACT_ID: &str = "b40678bd01d1b11e49569795caecf350c1d58826feb9996ee3f5784d05bf3909"; 
    const TESTNET_ADDRESS: &str = "wss://testnet.vara.network";


    let program_id = ProgramId::from(
        &hex::decode(MAIN_CONTRACT_ID.to_string()).unwrap()[..]
    );
    
    let address = WSAddress::new(TESTNET_ADDRESS, Some(443));

    let api = GearApi::init_with(address, MNEMONIC)
        .await
        .unwrap();


    println!("Connnected in vara network: {}", TESTNET_ADDRESS);

    let mut listener = api.subscribe().await.unwrap();

    assert!(listener.blocks_running().await.unwrap(), "Error, in blocks running method");


    println!("Sending actions to contract...");

    // Set weights first

    let  mut  payload = DoodleNNAction::SetAllWeights(Fraction::all_weights());

    send_message_and_wait_for_success(&api, &mut listener, program_id, payload).await;
    
    // Set biases second

    payload = DoodleNNAction::SetAllBiases(Fraction::all_bias());

    send_message_and_wait_for_success(&api, &mut listener, program_id, payload).await;

    

    // Send test prediction

    // payload = DoodleNNAction::Predict(Fraction::prediction_1());

    // send_message_and_wait_for_success(&api, &mut listener, program_id, payload).await;





    // // Set weights first

    // let mut payload = DoodleNNAction::SetWeights(Fraction::weights_first_first_data());

    // send_message_and_wait_for_success(&api, &mut listener, program_id, payload).await;

    // payload = DoodleNNAction::SetWeights(Fraction::weights_first_second_data());

    // send_message_and_wait_for_success(&api, &mut listener, program_id, payload).await;

    // payload = DoodleNNAction::AddDataInNextWeitght;

    // send_message_and_wait_for_success(&api, &mut listener, program_id, payload).await;

    // payload = DoodleNNAction::SetWeights(Fraction::weights_second_first_data());

    // send_message_and_wait_for_success(&api, &mut listener, program_id, payload).await;

    // // Set biases

    // payload = DoodleNNAction::SetBiases(Fraction::biases_first_first_data());

    // send_message_and_wait_for_success(&api, &mut listener, program_id, payload).await;

    // payload = DoodleNNAction::AddDataInNextBias;

    // send_message_and_wait_for_success(&api, &mut listener, program_id, payload).await;

    // payload = DoodleNNAction::SetBiases(Fraction::biases_second_first_data());

    // send_message_and_wait_for_success(&api, &mut listener, program_id, payload).await;

    // let payload = DoodleNNAction::Predict(Fraction::prediction_1());
    // send_message_and_wait_for_success(&api, &mut listener, program_id, payload).await;

    

    // println!("Message sent!!");

    // let t: Vec<String> = api.read_state(program_id, "".into()).await.unwrap();

    // print!("state:  {:?}", t);

    println!("Program end!");
}

async fn send_message_and_wait_for_success<E: Encode>(
    api: &GearApi,
    listener: &mut EventListener,
    pid: ProgramId,
    payload: E,
) -> MessageId {
    println!("Calculating gas...");

    let mut limit_gas_calculation = 10;

    let gas = loop {
        let gas_limit = api.calculate_handle_gas(
            None, 
            pid, 
            payload.encode(), 
            0, 
            true
        ).await;

        if let Ok(limit) = gas_limit {
            break limit.min_limit;
        } 

        if limit_gas_calculation == 0 {
            println!("Cant calculate gas!");
            return MessageId::default();
        }

        println!("Error calculando el gas fees");
        limit_gas_calculation -= 1;
    };

    println!("Gas fees to spend: {gas}");

    loop {
        match api.send_message_bytes(pid, payload.encode(), gas, 0).await {
            Ok((message_id, _)) => {
                if listener
                    .message_processed(message_id)
                    .await
                    .unwrap()
                    .failed()
                {
                    println!("ERROR: MSG NOT PROCESSED");
                    println!("Sending again the message...");
                    continue;
                }

                return message_id;
            }
            Err(e) => {
                println!("ERROR: {e}");
                return MessageId::default();
            }
        }
    }

    println!("Message was send!");
}


// fn get_num_of_data(mut num: u64) -> Vec<Fraction> {
//     let mut to_return = Vec::new();
//     let data = Fraction::weight_data();
//     for i in data.into_iter() {
//         for j in i.into_iter() {
//             for k in j.into_iter() {
//                 if num <= 0 {
//                     return to_return;
//                 }
//                 to_return.push(k);
//                 num -= 1;
//             }
//         }
//     }
//     to_return
// }

// #[tokio::main]
// async fn main() -> Result<()> {
//     // Open a connection to the mini-redis address.
//     let mut client = client::connect("127.0.0.1:6379").await?;

//     // Set the key "hello" with value "world"
//     client.set("hello", "world".into()).await?;

//     // Get key "hello"
//     let result = client.get("hello").await?;

//     println!("got value from the server; result={:?}", result);

//     Ok(())
// }



