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
use gstd::{Encode, CodeId};
use hex_literal;
use hex_fmt::HexFmt;

#[tokio::main]
async fn main() {
    const MNEMONIC: &str = "strong orchard plastic arena pyramid lobster lonely rich stomach label clog rubber";
    const MAIN_CONTRACT_ID: &str = "4c2f4974f6c9b2848dc92391902b3df8211cc9a3e017fb1f35657bb50b8f9c4e";
    const TESTNET_ADDRESS: &str = "wss://testnet.vara.network";
    
    let address = WSAddress::new(TESTNET_ADDRESS, Some(443));

    let program_id = ProgramId::from(
        &hex::decode(MAIN_CONTRACT_ID.to_string()).unwrap()[..]
    );

    let api = GearApi::init_with(address, MNEMONIC)
        .await
        .unwrap();

    println!("Connnected in vara network: {}", TESTNET_ADDRESS);

    let mut listener = api.subscribe().await.unwrap();

    assert!(listener.blocks_running().await.unwrap(), "Error, in blocks running method");

    let t: Vec<String> = api.read_state(program_id, "".into()).await.unwrap();

    print!("state:  {:?}", t);

    println!("Sending action to contract...");

    let payload = String::from("hola");

    send_message_and_wait_for_success(&api, &mut listener, program_id, payload).await;

    println!("Message sent!!");
}

async fn send_message_and_wait_for_success<E: Encode>(
    api: &GearApi,
    listener: &mut EventListener,
    pid: ProgramId,
    payload: E,
) -> MessageId {

    let gas_fee = while Err(_) {
        match api
            .calculate_handle_gas(None, pid, payload.encode(), 0, true)
            .await
        {
            Ok(limit) =>break  limit.min_limit,
            Err(_) => break api.block_gas_limit().unwrap(),
        };
    };

    loop {
        let gas = match api
            .calculate_handle_gas(None, pid, payload.encode(), 0, true)
            .await
        {
            Ok(limit) => limit.min_limit,
            Err(_) => api.block_gas_limit().unwrap(),
        };

        println!("Sending again the message");

        match api.send_message_bytes(pid, payload.encode(), gas, 0).await {
            Ok((message_id, _)) => {
                if listener
                    .message_processed(message_id)
                    .await
                    .unwrap()
                    .failed()
                {
                    println!("ERROR: MSG NOT PROCESSED");

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
}

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
