mod create_nft_token;
mod sell_nft_token;
mod token_data;
use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, program_error::ProgramError,
    pubkey::Pubkey,
};
mod buy_nft_token;
mod cancel_nft_token;
mod upgrade_nft_token;
fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    match instruction_data[0] {
        0 => create_nft_token::create_nft_token(
            program_id,
            accounts,
            &instruction_data[1..instruction_data.len()],
        ),
        1 => sell_nft_token::sell_nft_token(
            program_id,
            accounts,
            &instruction_data[1..instruction_data.len()],
        ),
        2 => buy_nft_token::but_nft_token(
            program_id,
            accounts,
            &instruction_data[1..instruction_data.len()],
        ),
        3 => cancel_nft_token::cancel_nft_sale(
            program_id,
            accounts,
            &instruction_data[1..instruction_data.len()],
        ),
        4 => upgrade_nft_token::upgrade_nft_token(
            program_id,
            accounts,
            &instruction_data[1..instruction_data.len()],
        ),
        _ => {
            return Err(ProgramError::InvalidInstructionData);
        }
    }
}
entrypoint!(process_instruction);

// Sanity tests
#[cfg(test)]
mod test {

    #[test]
    fn test_sanity() {}
}
