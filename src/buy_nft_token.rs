use std::str::FromStr;

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
    let token_account = next_account_info(accounts_iter)?;
    let token_program_id = next_account_info(accounts_iter)?;
    let owners_account = next_account_info(accounts_iter)?;
    let pda_account = next_account_info(accounts_iter)?;

    msg!("Buy_nft_token");
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

    if !data_present.is_for_sale {
        msg!("Not for sale");
        return Err(ProgramError::InvalidAccountData);
    }
    data_present.is_for_sale = false;
    if **pay_with.try_borrow_lamports()? != data_present.price as u64 {
        msg!("Insufficent balance");
        return Err(ProgramError::InsufficientFunds);
    }
    if data_present.owner == *signer.key {
        msg!("You can use cancel button");
        return Err(ProgramError::InvalidAccountData);
    }

    if **pay_with.try_borrow_lamports()? != data_present.price as u64 {
        msg!("Not enough lamport");
        return Err(ProgramError::InsufficientFunds);
    }
    let admin1 = Pubkey::from_str("3KNyVqUuQkfr2V1BAShtYfcZiREyVaPTtjPAQfbZSUV2")
        .expect("Failed to convert the pub key admin1");

    let admin2 = Pubkey::from_str("DGqXoguiJnAy8ExJe9NuZpWrnQMCV14SdEdiMEdCfpmB")
        .expect("Failed to convert the pub key admin2");

    if !(data_present.owner != *owners_account.key
        || data_present.owner != admin1
        || data_present.owner != admin2)
    {
        msg!("Wrong owners account");
        return Err(ProgramError::InvalidAccountData);
    }
    let (pda, bump_seed) =
        Pubkey::find_program_address(&[data_present.seed.as_bytes()], program_id);
    if &pda != pda_account.key {
        msg!("unexpected account");
    }

    let instruction = spl_token::instruction::set_authority(
        token_program_id.key,
        token_account.key,
        Some(signer.key),
        spl_token::instruction::AuthorityType::AccountOwner,
        &pda,
        &[&pda],
    )?;

    invoke_signed(
        &instruction,
        &[
            token_program_id.to_owned(),
            signer.to_owned(),
            writing_account.to_owned(),
            token_account.to_owned(),
            pda_account.to_owned(),
        ],
        &[&[&data_present.seed.as_bytes()[..], &[bump_seed]]], 
    )?;

    **owners_account.try_borrow_mut_lamports()? += **pay_with.try_borrow_lamports()?;
    **pay_with.try_borrow_mut_lamports()? = 0;

    data_present.owner = *signer.key;
    data_present.serialize(&mut &mut writing_account.data.borrow_mut()[..])?;
    Ok(())
}
