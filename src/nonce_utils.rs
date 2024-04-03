use crate::compute_unit_price::WithComputeUnitPrice;
use solana_client::{
    nonblocking::rpc_client::RpcClient as AsyncRpcClient,
    nonce_utils::{
        data_from_account, get_account_with_commitment,
        nonblocking::get_account_with_commitment as async_get_account_with_commitment,
    },
    rpc_client::RpcClient,
};
use solana_sdk::{
    nonce::state::Data,
    pubkey::Pubkey,
    signature::Keypair,
    signer::EncodableKeypair,
    system_instruction::create_nonce_account,
    transaction::Transaction,
};

const MINIMUM_RENT_LAMPORTS: u64 = 1447680_u64;

pub fn create_nonce_transaction_sync(
    rpc_client: &RpcClient,
    user_keypair: &Keypair,
) -> Result<(Transaction, Pubkey), solana_client::client_error::ClientError> {
    let nonce_keypair = Keypair::new();

    let create_nonce_instructions = create_nonce_account(
        &user_keypair.encodable_pubkey(),
        &nonce_keypair.encodable_pubkey(),
        &user_keypair.encodable_pubkey(),
        MINIMUM_RENT_LAMPORTS,
    )
    .with_compute_unit_price(Some(&500_000_u64));

    let mut transaction = Transaction::new_with_payer(
        &create_nonce_instructions.as_slice(),
        Some(&user_keypair.encodable_pubkey()),
    );

    let latest_blockhash = rpc_client.get_latest_blockhash()?;
    transaction.sign(&[&nonce_keypair, user_keypair], latest_blockhash);

    Ok((transaction, nonce_keypair.encodable_pubkey()))
}

pub async fn create_nonce_transaction_async(
    rpc_client: &AsyncRpcClient,
    user_pubkey: &Pubkey,
) -> Result<(Transaction, Pubkey), solana_client::client_error::ClientError> {
    let nonce_keypair = Keypair::new();

    let create_nonce_instructions = create_nonce_account(
        &user_pubkey,
        &nonce_keypair.encodable_pubkey(),
        &user_pubkey,
        MINIMUM_RENT_LAMPORTS,
    )
    .with_compute_unit_price(Some(&500_000_u64));
    let mut transaction =
        Transaction::new_with_payer(&create_nonce_instructions.as_slice(), Some(&user_pubkey));

    let latest_blockhash = rpc_client.get_latest_blockhash().await?;
    transaction.sign(&[&nonce_keypair], latest_blockhash);

    Ok((transaction, nonce_keypair.encodable_pubkey()))
}

pub async fn get_nonce_account_async(
    rpc_client: &AsyncRpcClient,
    nonce_account_pubkey: &Pubkey,
) -> Result<Data, solana_client::nonce_utils::Error> {
    let account_info = async_get_account_with_commitment(
        rpc_client,
        nonce_account_pubkey,
        rpc_client.commitment(),
    )
    .await?;
    data_from_account(&account_info)
}

pub fn get_nonce_account_sync(
    rpc_client: &RpcClient,
    nonce_account_pubkey: &Pubkey,
) -> Result<Data, solana_client::nonce_utils::Error> {
    let account_info =
        get_account_with_commitment(rpc_client, nonce_account_pubkey, rpc_client.commitment())?;
    data_from_account(&account_info)
}
