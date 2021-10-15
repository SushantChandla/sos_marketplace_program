use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::invoke,
    program_error::ProgramError,
    pubkey::Pubkey,
};

use crate::token_data::TokenData;

pub fn upgrade_nft_token(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let writing_account1 = next_account_info(accounts_iter)?;
    let token_account1 = next_account_info(accounts_iter)?;
    let writing_account2 = next_account_info(accounts_iter)?;
    let token_account2 = next_account_info(accounts_iter)?;
    let signer_account = next_account_info(accounts_iter)?;
    let spl_token_account = next_account_info(accounts_iter)?;
    if writing_account1.owner != program_id || writing_account2.owner != program_id {
        msg!("Transaction not possible account owners is not program account");
        return Err(ProgramError::InvalidAccountData);
    }
    if !signer_account.is_signer {
        msg!("Can't proceed, not signer");
        return Err(ProgramError::InvalidAccountData);
    }
    let mut data_present1 = TokenData::try_from_slice(*writing_account1.try_borrow_data()?)
        .expect("Failed to get the token data");
    let mut data_present2 = TokenData::try_from_slice(*writing_account2.try_borrow_data()?)
        .expect("Failed to get the token data");

    if data_present1.owner != data_present2.owner || data_present1.owner != *signer_account.key {
        msg!("Owned by differnt accounts, or invalid signer");
        return Err(ProgramError::InvalidAccountData);
    }
    if data_present1.name != data_present2.name || data_present1.category != data_present2.category
    {
        msg!("Cards are not same");
        return Err(ProgramError::InvalidInstructionData);
    }
    if data_present1.is_for_sale || data_present2.is_for_sale {
        msg!("Either cards are for sale, cancel first");
        return Err(ProgramError::InvalidInstructionData);
    }

    data_present1.level = std::cmp::max(data_present1.level, data_present2.level) + 1;
    data_present1.health = std::cmp::max(data_present1.health, data_present2.health) + 10;
    data_present1.attackpoints =
        std::cmp::max(data_present1.attackpoints, data_present2.attackpoints) + 10;

    data_present2.owner = *program_id;

    data_present2.serialize(&mut &mut writing_account1.data.borrow_mut()[..])?;

    let burn_instruction = spl_token::instruction::close_account(
        spl_token_account.key,
        token_account2.key,
        token_account1.key,
        signer_account.key,
        &[signer_account.key],
    )?;

    invoke(
        &burn_instruction,
        &[
            spl_token_account.to_owned(),
            writing_account1.to_owned(),
            signer_account.to_owned(),
            token_account1.to_owned()
        ],
    )?;

    data_present1.serialize(&mut &mut writing_account1.data.borrow_mut()[..])?;
    Ok(())
}
