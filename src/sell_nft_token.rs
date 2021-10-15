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

#[derive(BorshSerialize, BorshDeserialize, Debug)]
struct SellFor(u32);

pub fn sell_nft_token(
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
    let mut data_present = TokenData::try_from_slice(*writing_account.try_borrow_data()?)
        .expect("Failed to get the token data");

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
            token_account.to_owned(),
            writing_account.to_owned(),
            signer.to_owned(),
        ],
    )?;

    let input_data = SellFor::try_from_slice(instruction_data).expect("invalid instruction data");
    data_present.is_for_sale = true;
    data_present.price = input_data.0;
    input_data.serialize(&mut &mut writing_account.data.borrow_mut()[..])?;

    Ok(())
}
