use solana_client::rpc_client::RpcClient;
use solana_sdk::signature::{Keypair, Signer, read_keypair_file};

const RPC_URL: &str = "https://api.devnet.solana.com";

fn main() {
    // Load the keypair
    let keypair = read_keypair_file("dev_wallet.json").expect("Couldn't find wallet file");
    println!("{}", keypair.pubkey());
    // Connect to Solana Devnet
    let client = RpcClient::new(RPC_URL);

    // Request Airdrop of 2 SOL (2 billion lamports)
    let tx = client.request_airdrop(&keypair.pubkey(), 2_000_000_000u64);

    match tx {
        Ok(signature) => {
            println!("Airdrop successful! Check your transaction here:");
            println!("https://explorer.solana.com/tx/{}?cluster=devnet", signature);

            // Wait for transaction confirmation
            match client.confirm_transaction(&signature) {
                Ok(data) =>   if data {
                    println!("Transaction confirmed!");
                    
                }else{
                    println!("Transaction not confirmed!");
                },
                
                Err(e) => println!("Transaction might not have been confirmed yet: {}", e),
            }
        }
        Err(e) => println!("Airdrop request failed: {}", e),
    }
}
