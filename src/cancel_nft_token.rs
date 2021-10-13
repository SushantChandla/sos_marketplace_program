
use crate::token_data::TokenData;
use borsh::{BorshDeserialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

pub fn cancel_nft_sale(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let writing_account = next_account_info(accounts_iter)?;
    let signer = next_account_info(accounts_iter)?;
    let mint_account = next_account_info(accounts_iter)?;
    if writing_account.owner != program_id {
        msg!("Writter account isn't owned by program");
        return Err(ProgramError::IncorrectProgramId);
    }

    if !signer.is_signer {
        msg!("Signer error");
        return Err(ProgramError::MissingRequiredSignature);
    }
    let mut data_present = TokenData::try_from_slice(*writing_account.try_borrow_data()?)
        .expect("Failed to get the token data");

    if data_present.mint_id != *mint_account.key {
        msg!("Invalid Instruction data data_present.mint_id != *mint_account.key");
        return Err(ProgramError::InvalidAccountData);
    }
    if signer.key!=mint_account.owner{
        msg!("Is not owned");
        return Err(ProgramError::InvalidAccountData);
    }

    if data_present.is_for_sale!=true{
        
    }
    

    Ok(())
}