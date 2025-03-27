use solana_sdk::{
    signature::{Keypair, Signer, read_keypair_file},
    transaction::Transaction,
    pubkey::Pubkey,
};
use solana_client::rpc_client::RpcClient;
use std::str::FromStr;

const RPC_URL: &str = "https://api.devnet.solana.com";

fn main() {
    // Import our keypair
    let keypair = read_keypair_file("dev_wallet.json").expect("Couldn't find wallet file");
    
    // Define recipient public key (Replace with actual Turbin3 wallet address)
    let to_pubkey = Pubkey::from_str("86PBv9rEowvUYAhdDzNWgKkVKNc3ESGiZKtjheS9JBs6").unwrap();

    // Create a Solana devnet connection
    let rpc_client = RpcClient::new(RPC_URL);

    // Get recent blockhash
    let recent_blockhash = rpc_client.get_latest_blockhash().expect("Failed to get recent blockhash");

    // Create and sign the transaction (transferring 0.1 SOL = 100_000_000 lamports)
    let transaction = Transaction::new_signed_with_payer(
        &[solana_sdk::system_instruction::transfer(
            &keypair.pubkey(),
            &to_pubkey,
            100_000_000, // 0.1 SOL in lamports
        )],
        Some(&keypair.pubkey()),
        &vec![&keypair],
        recent_blockhash,
    );

    // Send the transaction
    let signature = rpc_client.send_and_confirm_transaction(&transaction)
        .expect("Failed to send transaction");

    // Print the transaction link
    println!(
        "Success! Check out your TX here: https://explorer.solana.com/tx/{}/?cluster=devnet",
        signature
    );
}
