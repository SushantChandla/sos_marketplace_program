use std::str::FromStr;

use crate::token_data::TokenData;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::invoke,
    program_error::ProgramError,
    pubkey::Pubkey,
};

pub fn create_nft_token(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let writing_account = next_account_info(accounts_iter)?;
    let signer = next_account_info(accounts_iter)?;
    let token_account = next_account_info(accounts_iter)?;
    let spl_token_account = next_account_info(accounts_iter)?;

    if writing_account.owner != program_id {
        msg!("Writter account isn't owned by program");
        return Err(ProgramError::IncorrectProgramId);
    }

    if !signer.is_signer {
        msg!("Signer error");
        return Err(ProgramError::MissingRequiredSignature);
    }

    let admin1 = Pubkey::from_str("3KNyVqUuQkfr2V1BAShtYfcZiREyVaPTtjPAQfbZSUV2")
        .expect("Failed to convert the pub key admin1");

    let admin2 = Pubkey::from_str("DGqXoguiJnAy8ExJe9NuZpWrnQMCV14SdEdiMEdCfpmB")
        .expect("Failed to convert the pub key admin2");
    if !(*signer.key == admin1 || *signer.key == admin2) {
        msg!("Signer not admin");
        return Err(ProgramError::IllegalOwner);
    }

    let mut input_data =
        TokenData::try_from_slice(instruction_data).expect("Failed to convert the input data");

    let set_update_auth = spl_token::instruction::set_authority(
        spl_token_account.key,
        token_account.key,
        Some(writing_account.key),
        spl_token::instruction::AuthorityType::AccountOwner,
        signer.key,
        &[signer.key],
    )?;

    invoke(
        &set_update_auth,
        &[
            spl_token_account.to_owned(),
            signer.to_owned(),
            writing_account.to_owned(),
            token_account.to_owned(),
        ],
    )?;

    msg!("set_update_auth passed");

    input_data.level = 1;
    input_data.is_for_sale = true;
    input_data.owner = *signer.key;

    input_data.serialize(&mut &mut writing_account.data.borrow_mut()[..])?;
    Ok(())
}
