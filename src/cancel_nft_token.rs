use crate::token_data::TokenData;
use borsh::BorshDeserialize;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::invoke_signed,
    program_error::ProgramError,
    pubkey::Pubkey,
};

pub fn cancel_nft_sale(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let writing_account = next_account_info(accounts_iter)?;
    let signer = next_account_info(accounts_iter)?;
    let token_account = next_account_info(accounts_iter)?;
    let spl_token_account = next_account_info(accounts_iter)?;
    let pda_account = next_account_info(accounts_iter)?;
    if writing_account.owner != program_id {
        msg!("Writter account isn't owned by program");
        return Err(ProgramError::IncorrectProgramId);
    }

    // if !signer.is_signer {
    //     msg!("Signer error");
    //     return Err(ProgramError::MissingRequiredSignature);
    // }
    let data_present = TokenData::try_from_slice(*writing_account.try_borrow_data()?)
        .expect("Failed to get the token data");

    if data_present.owner != *signer.key {
        msg!("Do not own the key can't cancel");
        return Err(ProgramError::InvalidAccountData);
    }

    let (pda, bump_seed) =
        Pubkey::find_program_address(&[data_present.seed.as_bytes()], program_id);

    let set_update_auth = spl_token::instruction::set_authority(
        spl_token_account.key,
        token_account.key,
        Some(signer.key),
        spl_token::instruction::AuthorityType::AccountOwner,
        &pda,
        &[&pda],
    )?;

    invoke_signed(
        &set_update_auth,
        &[
            spl_token_account.to_owned(),
            token_account.to_owned(),
            writing_account.to_owned(),
            pda_account.to_owned(),
        ],
        &[&[&data_present.seed.as_bytes()[..], &[bump_seed]]],
    )?;

    Ok(())
}
