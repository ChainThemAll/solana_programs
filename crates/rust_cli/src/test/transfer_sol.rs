#[test]
pub fn test_lamport_transfer() {
    use borsh::BorshSerialize;
    const RPC_ADDR: &str = "http://127.0.0.1:8899";
    use solana_program::pubkey::Pubkey;
    use solana_rpc_client::rpc_client::RpcClient;
    use solana_sdk::signature::{read_keypair_file, Signer};
    use solana_sdk::transaction::Transaction;
    use std::path::PathBuf;
    use std::str::FromStr;
    use transfer_sol::processor::TransferInstruction;
    let transfer_amount: u64 = 1000; // Specify the amount of lamports to transfer

    let program_id = Pubkey::from_str("8czkmWK83eTuhfkYit6rZFMf7yb2nXQxusK5a4JcD2W7").unwrap();

    let keypair_path = PathBuf::from("/home/pomelo/.config/solana/id.json");
    let payer_keypair =
        read_keypair_file(keypair_path).expect("Failed to read payer keypair from file");

    let client = RpcClient::new(RPC_ADDR);

    // Create account metas
    let account_metas = vec![
        solana_program::instruction::AccountMeta::new(payer_keypair.pubkey(), true),
        solana_program::instruction::AccountMeta::new(payer_keypair.pubkey(), false),
    ];

    // Create instruction
    let mut data = Vec::new();
    TransferInstruction::ProgramTransfer(transfer_amount)
        .serialize(&mut data)
        .unwrap();

    // Assuming the instruction data just contains the amount
    let instruction =
        solana_program::instruction::Instruction::new_with_bytes(program_id, &data, account_metas);

    // Create and send transaction
    let ixs = vec![instruction];
    let latest_blockhash = client.get_latest_blockhash().unwrap();
    let sig = client
        .send_and_confirm_transaction(&Transaction::new_signed_with_payer(
            &ixs,
            Some(&payer_keypair.pubkey()),
            &[&payer_keypair],
            latest_blockhash,
        ))
        .unwrap();

    println!("Transaction signature: {}", sig);
}
