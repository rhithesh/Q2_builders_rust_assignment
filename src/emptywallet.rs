use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    message::Message,
    pubkey::Pubkey,
    signature::{Keypair, Signer, read_keypair_file},
    system_instruction::transfer,
    transaction::Transaction,
};
use std::str::FromStr;

const RPC_URL: &str = "https://api.devnet.solana.com";

fn main() {
    // Load keypair
    let keypair = read_keypair_file("dev_wallet.json").expect("Couldn't find wallet file");
    let from_pubkey = keypair.pubkey();

    // Define recipient public key (replace with the actual destination wallet address)
    let to_pubkey = Pubkey::from_str("GLtaTaYiTQrgz411iPJD79rsoee59HhEy18rtRdrhEUJ").unwrap();

    // Connect to Solana Devnet
    let rpc_client = RpcClient::new(RPC_URL);

    // Get balance of the sender account
    let balance = rpc_client
        .get_balance(&from_pubkey)
        .expect("Failed to get balance");

    if balance == 0 {
        println!("No SOL available in the account to transfer.");
        return;
    }

    // Get recent blockhash
    let recent_blockhash = rpc_client
        .get_latest_blockhash()
        .expect("Failed to get recent blockhash");

    // Create a test transaction to estimate transaction fee
    let message = Message::new_with_blockhash(
        &[transfer(&from_pubkey, &to_pubkey, balance)], 
        Some(&from_pubkey), 
        &recent_blockhash,
    );

    // Get fee for the mock transaction
    let fee = rpc_client
        .get_fee_for_message(&message)
        .expect("Failed to get fee calculator");

    if balance <= fee {
        println!("Not enough SOL to cover transaction fees.");
        return;
    }

    // Deduct fee and create a real transaction
    let transaction = Transaction::new_signed_with_payer(
        &[transfer(&from_pubkey, &to_pubkey, balance - fee)], 
        Some(&from_pubkey), 
        &vec![&keypair], 
        recent_blockhash,
    );

    // Send transaction
    let signature = rpc_client
        .send_and_confirm_transaction(&transaction)
        .expect("Failed to send transaction");

    // Print the transaction link
    println!(
        "Success! Check out your TX here: https://explorer.solana.com/tx/{}/?cluster=devnet",
        signature
    );
}
