use borsh::BorshDeserialize;
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;

pub enum ZilchInstruction {
    AddMovieReview {
        title: String,
        rating: u8,
        description: String,
    },
    AddProof {
        proghash: String,
        proof_account: Pubkey,
        outputs: String,
    },
}

#[derive(BorshDeserialize)]
struct ZilchReviewPayload {
    hash: String,
    proof: Pubkey,
    program: String,
    inputs: u8,
    outputs: String,
}

impl ZilchInstruction {
    // Unpack inbound buffer to associated Instruction
    // The expected format for input is a Borsh serialized vector
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        // Split the first byte of data
        let (&variant, rest) = input
            .split_first()
            .ok_or(ProgramError::InvalidInstructionData)?;
        // `try_from_slice` is one of the implementations from the BorshDeserialization trait
        // Deserializes instruction byte data into the payload struct
        let payload = ZilchReviewPayload::try_from_slice(rest).unwrap();
        // Match the first byte and return the AddMovieReview struct
        Ok(match variant {
            0 => Self::AddMovieReview {
                title: payload.hash,
                rating: payload.inputs,
                description: payload.program,
            },
            1 => Self::AddProof {
                proghash: payload.hash,
                proof_account: payload.proof,
                outputs: payload.outputs,
            },
            _ => return Err(ProgramError::InvalidInstructionData),
        })
    }
}
