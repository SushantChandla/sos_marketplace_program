use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::invoke_signed,
    program_error::ProgramError,
    pubkey::Pubkey,
};

use crate::token_data::TokenData;

pub fn but_nft_token(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let writing_account = next_account_info(accounts_iter)?;
    let signer = next_account_info(accounts_iter)?;
    let pay_with = next_account_info(accounts_iter)?;
    let mint_account = next_account_info(accounts_iter)?;

    let token_program_id = next_account_info(accounts_iter)?;
    let source_account = next_account_info(accounts_iter)?;

    if writing_account.owner != program_id || pay_with.owner != program_id {
        msg!("Writter account or pay_with isn't owned by program");
        return Err(ProgramError::IncorrectProgramId);
    }
    if !signer.is_signer {
        msg!("Signer error");
        return Err(ProgramError::MissingRequiredSignature);
    }
    let mut data_present = TokenData::try_from_slice(*writing_account.try_borrow_data()?)
        .expect("Failed to get the token data");
    if data_present.mint_id != *mint_account.key {
        msg!("Invalid account data");
        return Err(ProgramError::InvalidAccountData);
    }
    if !data_present.is_for_sale {
        msg!("Not for sale");
        return Err(ProgramError::InvalidAccountData);
    }
    data_present.is_for_sale = false;
    if **pay_with.try_borrow_lamports()? == data_present.price as u64 {
        msg!("Insufficent balance");
        return Err(ProgramError::InsufficientFunds);
    }

    let instruction = spl_token::instruction::transfer(
        token_program_id.key,
        source_account.key,
        signer.key,
        &writing_account.key,
        &[],
        1,
    )?;
    invoke_signed(
        &instruction,
        &[
            source_account.to_owned(),
            signer.to_owned(),
            token_program_id.to_owned(),
            writing_account.to_owned(),
        ],
        &[&["carddata".as_bytes()]],
    )?;

    data_present.serialize(&mut &mut writing_account.data.borrow_mut()[..])?;
    Ok(())
}
