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
    let mint_account = next_account_info(accounts_iter)?;
    let spl_token_account = next_account_info(accounts_iter)?;
    if writing_account.owner != program_id {
        msg!("Writter account isn't owned by program");
        return Err(ProgramError::IncorrectProgramId);
    }

    if !signer.is_signer {
        msg!("Signer error");
        return Err(ProgramError::MissingRequiredSignature);
    }
    let data_present = TokenData::try_from_slice(*writing_account.try_borrow_data()?)
        .expect("Failed to get the token data");

    if data_present.mint_id != *mint_account.key {
        msg!("Invalid Instruction data data_present.mint_id != *mint_account.key");
        return Err(ProgramError::InvalidAccountData);
    }
    if *mint_account.owner != spl_token::id() {
        msg!("Is not spl token account owned");
        return Err(ProgramError::InvalidAccountData);
    }
    if data_present.owner != *signer.key{
        msg!("Do not own the key can't cancel");
        return Err(ProgramError::InvalidAccountData);
    }

    let set_update_auth = spl_token::instruction::set_authority(
        spl_token_account.key,
        mint_account.key,
        Some(signer.key),
        spl_token::instruction::AuthorityType::AccountOwner,
        writing_account.key,
        &[writing_account.key],
    )?;

    invoke_signed(
        &set_update_auth,
        &[spl_token_account.to_owned()],
        &[&["carddata".as_bytes()]],
    )?;

    Ok(())
}
