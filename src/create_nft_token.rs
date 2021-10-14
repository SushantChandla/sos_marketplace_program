use std::str::FromStr;

use crate::token_data::TokenData;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::{invoke, invoke_signed},
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
    let metadata_program = next_account_info(accounts_iter)?;
    let metadata_account = next_account_info(accounts_iter)?;
    let mint = next_account_info(accounts_iter)?;
    let spl_token_account = next_account_info(accounts_iter)?;
    let system_account = next_account_info(accounts_iter)?;
    let rent_account = next_account_info(accounts_iter)?;

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
        signer.key,
        Some(writing_account.key),
        spl_token::instruction::AuthorityType::AccountOwner,
        signer.key,
        &[signer.key],
    )?;
    invoke(&set_update_auth, &[spl_token_account.to_owned()])?;

    let instruction_for_metadata = spl_token_metadata::instruction::create_metadata_accounts(
        *metadata_program.key,
        input_data.metadata_at,
        input_data.mint_id,
        *writing_account.key,
        *writing_account.key,
        *writing_account.key,
        String::from("Shadow of Stroms"),
        String::from("SOS"),
        input_data.uri.to_owned(),
        Some(vec![
            spl_token_metadata::state::Creator {
                address: admin1,
                share: 50,
                verified: true,
            },
            spl_token_metadata::state::Creator {
                address: admin2,
                share: 50,
                verified: true,
            },
        ]),
        1000,
        true,
        true,
    );

    invoke_signed(
        &instruction_for_metadata,
        &[
            metadata_program.to_owned(),
            metadata_account.to_owned(),
            mint.to_owned(),
            writing_account.to_owned(),
            writing_account.to_owned(),
            writing_account.to_owned(),
            system_account.to_owned(),
            rent_account.to_owned(),
        ],
        &[&["carddata".as_bytes()]],
    )?;

    input_data.level = 1;
    input_data.is_for_sale = true;
    input_data.owner = *signer.key;

    input_data.serialize(&mut &mut writing_account.data.borrow_mut()[..])?;
    Ok(())
}
