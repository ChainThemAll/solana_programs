#[test]
pub fn test_hello_world() {
    const RPC_ADDR: &str = "http://127.0.0.1:8899";
    use std::str::FromStr;

    use solana_program::instruction;
    use solana_program::pubkey;
    use solana_rpc_client::rpc_client;
    use solana_sdk::signature::read_keypair_file;
    use solana_sdk::signature::Signer;
    use solana_sdk::transaction;
    use std::path::PathBuf;
    println!("0");
    let helloworld =
        pubkey::Pubkey::from_str("EDwvUS7eJCzmup8BdJpYVUehk82aDc8qzbCWN4kUtfgB").unwrap();

    let keypair_path = PathBuf::from("/home/alain/.config/solana/id.json");

    // 从文件中读取密钥对
    let me = read_keypair_file(keypair_path).expect("Failed to read keypair from file");

    println!("me is {}", me.pubkey());

    let client = rpc_client::RpcClient::new(RPC_ADDR);
    println!("1");

    let account_metas = vec![instruction::AccountMeta::new(me.pubkey(), true)];

    let instruction =
        instruction::Instruction::new_with_bytes(helloworld, "hello".as_bytes(), account_metas);
    let ixs = vec![instruction];
    println!("2");
    let latest_blockhash = client.get_latest_blockhash().unwrap();
    println!("3");
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
