pub mod compute_unit_price;
pub mod nonce_utils;
pub mod use_durable_nonce;
pub mod use_storage;

pub use {compute_unit_price::*, nonce_utils::*, use_durable_nonce::*, use_storage::*};
use {
    solana_client::rpc_client::RpcClient,
    solana_sdk::{
        commitment_config::CommitmentConfig, instruction::Instruction, message::Message,
        pubkey::Pubkey, signature::read_keypair_file, signer::EncodableKeypair, system_instruction,
        transaction::Transaction,
    },
    std::str::FromStr,
};

/// tested in devnet
fn main() -> Result<(), std::io::Error> {
    let payer = read_keypair_file(&"test_keypair.json").unwrap();
    const API_ENDPOINT: &str = "";
    let rpc_client =
        RpcClient::new_with_commitment(API_ENDPOINT.to_string(), CommitmentConfig::confirmed());

    // first we create nonce_account
    create_nonce_account(&rpc_client, &payer);

    // second run the transaction
    run_transaction(payer, rpc_client);
    Ok(())
}

fn run_transaction(payer: solana_sdk::signature::Keypair, rpc_client: RpcClient) {
    // below will fail if it doesn't exist due to unwrap()
    let durable_nonce_string =
        get_durable_nonce(payer.encodable_pubkey().to_string().as_str()).unwrap();
    let durable_nonce_account_key = Pubkey::from_str(durable_nonce_string.as_str()).unwrap();

    // any instruction
    let mut instructions = Vec::<Instruction>::new();
    let instr_transfer = system_instruction::transfer(
        &payer.encodable_pubkey(),
        &Pubkey::from_str("").unwrap(),
        1_u64,
    );
    instructions.push(instr_transfer);

    // add prio fees
    let instructions_with_priority_fees = instructions.with_compute_unit_price(Some(&500_000_u64));

    // message with new nonce must be used otherwise it will say Blockhash not found
    let message = Message::new_with_nonce(
        instructions_with_priority_fees,
        Some(&payer.encodable_pubkey()),
        &durable_nonce_account_key,
        &payer.encodable_pubkey(),
    );
    let nonce_account = get_nonce_account_sync(&rpc_client, &durable_nonce_account_key).unwrap();
    let blockhash = nonce_account.blockhash();
    let mut tx = Transaction::new(&[&payer], message, blockhash);

    tx.sign(&[&payer], blockhash);
    let result = rpc_client.send_and_confirm_transaction_with_spinner(&tx);

    match result {
        Ok(signature) => {
            print!("{}", signature);
        }
        Err(err) => {
            eprintln!("Error sending transaction: {}", err);
        }
    }
}

fn create_nonce_account(rpc_client: &RpcClient, payer: &solana_sdk::signature::Keypair) {
    let (txn, nonce_pubkey) = create_nonce_transaction_sync(rpc_client, payer).unwrap();

    let result = rpc_client.send_and_confirm_transaction_with_spinner(&txn);

    match result {
        Ok(signature) => {
            let payer_pubkey_str = payer.encodable_pubkey().to_string();
            let nonce_pubkey_str = nonce_pubkey.to_string();
            print!("{}", signature);
            set_durable_nonce(&payer_pubkey_str, &nonce_pubkey_str);
        }
        Err(err) => {
            eprintln!("Error sending transaction: {}", err);
        }
    }
}
