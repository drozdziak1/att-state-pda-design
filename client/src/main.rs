use log::*;
use solana_client::rpc_client::RpcClient;
use solana_program::instruction::AccountMeta;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    instruction::Instruction,
    pubkey::Pubkey,
    signature::{read_keypair_file, Signer},
    signer::keypair::Keypair,
    transaction::Transaction,
};

use att_state_pda_design::AttestationStatePDA;
use solitaire::{processors::seeded::Seeded, BorshSerialize};

use std::str::FromStr;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::builder()
        .filter_level(LevelFilter::Trace)
        .init();

    let c = RpcClient::new_with_commitment(
        "http://localhost:8899".to_owned(),
        CommitmentConfig::confirmed(),
    );

    let payer = read_keypair_file("/home/drozdziak1/.config/solana/id.json")?;
    let payer_pubkey = payer.pubkey();

    let program_id: Pubkey = Pubkey::from_str("Attest1p5gheXUvJ6jGWGeCsgPKgnE3YgdGKRVCMY9o")?;

    let pda_addr = AttestationStatePDA::key(None, &program_id);

    println!("PDA address is {}", pda_addr.to_string());
    println!("Payer address is {}", payer_pubkey.to_string());

    let airdrop_sig = c.request_airdrop(&payer_pubkey, 1_000_000_000u64)?; // 10 SOL

    c.confirm_transaction_with_commitment(&airdrop_sig, CommitmentConfig::confirmed())?;

    let metas = vec![
        AccountMeta::new(payer_pubkey.clone(), true),
        AccountMeta::new(pda_addr, false),
        AccountMeta::new(solana_program::system_program::id(), false),
    ];

    let ix = Instruction::new_with_bytes(
        program_id,
        (
            att_state_pda_design::instruction::Instruction::Testing,
            ()
        ).try_to_vec()?.as_slice(),
        metas,
    );

    let signers = vec![&payer];

    let blockhash = c.get_latest_blockhash()?;

    let tx_signed = Transaction::new_signed_with_payer::<Vec<&Keypair>>(
        &[ix],
        Some(&payer_pubkey),
        &signers,
        blockhash,
    );

    info!("before send_transaction");

    c.send_transaction(&tx_signed)?;

    Ok(())
}
