pub mod programs;

use solana_sdk::{
    signature::{Keypair, Signer, read_keypair_file}, 
    pubkey::Pubkey
};
use solana_client::rpc_client::RpcClient;
use bs58;
use std::io::{self, BufRead};
use crate::programs::Turbin3_prereq::{TurbinePrereqProgram, CompleteArgs};

pub fn generate_keypair() -> Keypair {
    Keypair::new()
}

use solana_sdk::{
    message::Message, native_token::LAMPORTS_PER_SOL, system_instruction::transfer,
    system_program, transaction::Transaction,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keygen() {
        let kp = generate_keypair();
        
        println!("You've generated a new Solana wallet: {}", kp.pubkey().to_string());
        println!("\nTo save your wallet, copy and paste the following into a JSON file:");
        println!("{:?}", kp.to_bytes());
    }

    #[test]
fn base58_to_wallet() {
println!("Input your private key as base58:");
let stdin = io::stdin();
let base58 = stdin.lock().lines().next().unwrap().unwrap(); println!("Your wallet file is:");
let wallet = bs58::decode(base58).into_vec().unwrap(); println!("{:?}", wallet);
}

#[test]
fn wallet_to_base58() {
println!("Input your private key as a wallet file byte array:");
let stdin = io::stdin();
let wallet =
stdin.lock().lines().next().unwrap().unwrap().trim_start_matches('[').trim_end_matches(']').
split(',') .map(|s| s.trim().parse::<u8>().unwrap()).collect::<Vec<u8>>();
println!("Your private key is:");
let base58 = bs58::encode(wallet).into_string(); println!("{:?}", base58);
}
}


#[test]
fn enroll() {
            let signer = read_keypair_file("/Users/hithesh/Desktop/airdroprust/src/important-wallet.json").expect("Couldn't find wallet file");
            let url = "https://api.devnet.solana.com";
            let rpc_client = RpcClient::new(url);

            let prereq = TurbinePrereqProgram::derive_program_address(&[b"prereq",
signer.pubkey().to_bytes().as_ref()]);

            let args = CompleteArgs {
                github: b"rhithesh".to_vec(),
            };

            let blockhash = rpc_client
                .get_latest_blockhash()
                .expect("Failed to get recent blockhas");

            let transaction = TurbinePrereqProgram::complete(
                &[&signer.pubkey(), &prereq, &system_program::id()],
                &args,
                Some(&signer.pubkey()),
                &[&signer],
                blockhash,
            );

            let signature = rpc_client
                .send_and_confirm_transaction(&transaction)
                .expect("Failed to send transaction");

            println!("Success! Check out your TX here:https://explorer.solana.com/tx/{}/?cluster=devnet", signature);
        }
      
    