use solana_program::pubkey;
use solana_sdk::{pubkey::Pubkey, signature::Keypair};

pub mod create_account;
pub mod hello_world;
pub mod transfer_sol;

pub fn test(rpc_addr: &str, program_id: Pubkey, wallet: Keypair) {}
