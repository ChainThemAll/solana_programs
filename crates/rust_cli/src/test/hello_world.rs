const RPC_ADDR: &str = "http://127.0.0.1:8899";
use std::str::FromStr;

use solana_program::instruction;
use solana_program::pubkey::Pubkey;
use solana_rpc_client::rpc_client;
use solana_sdk::signature::read_keypair_file;
use solana_sdk::signature::Keypair;
use solana_sdk::signature::Signer;
use solana_sdk::transaction;
use std::path::PathBuf;
#[test]
pub fn test_hello_world() {
    let helloworld = Pubkey::from_str("EDwvUS7eJCzmup8BdJpYVUehk82aDc8qzbCWN4kUtfgB").unwrap();

    let keypair_path = PathBuf::from("/home/alain/.config/solana/id.json");

    // 从文件中读取密钥对
    let me = read_keypair_file(keypair_path).expect("Failed to read keypair from file");

    println!("me is {}", me.pubkey());

    let client = rpc_client::RpcClient::new(RPC_ADDR);

    let account_metas = vec![instruction::AccountMeta::new(me.pubkey(), true)];

    let instruction =
        instruction::Instruction::new_with_bytes(helloworld, "hello".as_bytes(), account_metas);
    let ixs = vec![instruction];

    let latest_blockhash = client.get_latest_blockhash().unwrap();

    let sig = client
        .send_and_confirm_transaction(&transaction::Transaction::new_signed_with_payer(
            &ixs,
            Some(&me.pubkey()),
            &[&me],
            latest_blockhash,
        ))
        .unwrap();

    println!("tx:{}", sig);
}
pub fn test(rpc_addr: &str, program_id: Pubkey, wallet: Keypair) {
    let helloworld = Pubkey::from_str("EDwvUS7eJCzmup8BdJpYVUehk82aDc8qzbCWN4kUtfgB").unwrap();

    let keypair_path = PathBuf::from("/home/alain/.config/solana/id.json");

    // 从文件中读取密钥对
    let me = read_keypair_file(keypair_path).expect("Failed to read keypair from file");

    println!("me is {}", me.pubkey());

    let client = rpc_client::RpcClient::new(RPC_ADDR);

    let account_metas = vec![instruction::AccountMeta::new(me.pubkey(), true)];

    let instruction =
        instruction::Instruction::new_with_bytes(helloworld, "hello".as_bytes(), account_metas);
    let ixs = vec![instruction];

    let latest_blockhash = client.get_latest_blockhash().unwrap();

    let sig = client
        .send_and_confirm_transaction(&transaction::Transaction::new_signed_with_payer(
            &ixs,
            Some(&me.pubkey()),
            &[&me],
            latest_blockhash,
        ))
        .unwrap();

    println!("tx:{}", sig);
}
