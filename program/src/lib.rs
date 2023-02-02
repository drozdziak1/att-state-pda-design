use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::AccountInfo, clock::UnixTimestamp, entrypoint, entrypoint::ProgramResult,
    pubkey::Pubkey,
};
use solitaire::{
    create_account, trace, AccountOwner, AccountState, Creatable, CreationLamports, Data, Derive,
    ExecutionContext, FromAccounts, Info, Keyed, Mut, Owned, Peel, Seeded, Signer,
};

use std::collections::BTreeMap;

#[derive(BorshSerialize, BorshDeserialize)]
pub struct AttestationState {
    prev_timestamp: UnixTimestamp,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct AttestationStateMap {
    state: BTreeMap<Pubkey, AttestationState>,
}

impl Default for AttestationStateMap {
    fn default() -> Self {
        AttestationStateMap {
            state: BTreeMap::new(),
        }
    }
}

impl Owned for AttestationStateMap {
    fn owner(&self) -> AccountOwner {
        AccountOwner::This
    }
}

pub type AttestationStatePDA<'b> = Derive<
    Data<'b, AttestationStateMap, { AccountState::MaybeInitialized }>,
    "p2w-attestation-state-v1",
>;

#[derive(FromAccounts)]
pub struct Testing<'b> {
    payer: Mut<Signer<Info<'b>>>,
    attestation_state: Mut<AttestationStatePDA<'b>>,
    system_program: Info<'b>,
}

pub fn testing<'b>(
    ctxt: &ExecutionContext,
    accounts: &mut Testing<'b>,
    data: (),
) -> solitaire::Result<()> {
    let seeds = accounts.attestation_state.self_bumped_seeds(None, ctxt.program_id);

    trace!("seeds created");

    let seeds_ref = seeds.iter().map(|seeds| seeds.as_slice()).collect::<Vec<_>>();
    // if !accounts.attestation_state.is_initialized() {
    create_account(
        ctxt,
        accounts.attestation_state.info(),
        accounts.payer.key,
        CreationLamports::Exempt,
        1_024,
        ctxt.program_id,
        solitaire::IsSigned::SignedWithSeeds(&[seeds_ref.as_slice()]),
    )?;
    // }
    Ok(())
}

solitaire::solitaire! {Testing => testing}
