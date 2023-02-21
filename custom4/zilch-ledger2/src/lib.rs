use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, msg, pubkey::Pubkey,
};

pub mod instruction;
use instruction::ZilchInstruction;

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    // Unpack called
    let instruction = ZilchInstruction::unpack(instruction_data)?;
    // Match against the data struct returned into `instruction` variable
    match instruction {
        ZilchInstruction::AddMovieReview {
            title,
            rating,
            description,
        } => {
            // Make a call to `add_move_review` function
            add_compute_request(program_id, accounts, title, rating, description)
        }
        ZilchInstruction::AddProof {
            proghash,
            proof_account,
            outputs,
        } => add_proof(program_id, accounts, proghash, proof_account, outputs),
    }
}

pub fn add_proof(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    title: String,
    rating: Pubkey,
    description: String,
) -> ProgramResult {
    // Logging instruction data that was passed in
    msg!("Adding proof review...");
    msg!("Title: {}", title);
    msg!("Rating: {}", rating);
    msg!("Description: {}", description);

    Ok(())
}

pub fn add_compute_request(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    title: String,
    rating: u8,
    description: String,
) -> ProgramResult {
    // Logging instruction data that was passed in
    msg!("Adding comp rq...");
    msg!("Title: {}", title);
    msg!("Rating: {}", rating);
    msg!("Description: {}", description);

    Ok(())
}
