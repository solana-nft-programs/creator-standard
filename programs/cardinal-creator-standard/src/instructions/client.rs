// use crate::CreatorStandardInstruction;
// use borsh::BorshSerialize;
// use solana_program::instruction::AccountMeta;
// use solana_program::instruction::Instruction;
// use solana_program::program_error::ProgramError;
// use solana_program::pubkey::Pubkey;
// use solana_program::system_program;

// pub fn init_ix(
//     mint: &Pubkey,
//     authority: &Pubkey,
//     standard: &Pubkey,
// ) -> Result<Instruction, ProgramError> {
//     Ok(Instruction {
//         program_id: crate::id(),
//         accounts: vec![
//             AccountMeta::new(*mint, false),
//             AccountMeta::new(*authority, true),
//             AccountMeta::new(*standard, true),
//             AccountMeta::new_readonly(spl_token::id(), false),
//             AccountMeta::new_readonly(system_program::id(), false),
//         ],
//         data: CreatorStandardInstruction::InitMintManager.try_to_vec()?,
//     })
// }
