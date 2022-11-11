use borsh::BorshSerialize;
use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    sysvar,
};

use crate::CreatorStandardInstruction;

use super::{
    approve::ApproveIx,
    mint_manager::{SetInUseByIx, UpdateMintManagerIx},
    InitRulesetIx, UpdateRulesetIx,
};

//////// ruleset ////////
#[allow(clippy::too_many_arguments)]
pub fn init_ruleset(
    program_id: Pubkey,
    ruleset: Pubkey,
    authority: Pubkey,
    payer: Pubkey,
    name: String,
    collector: Pubkey,
    disallowed_addresses: Vec<Pubkey>,
    allowed_programs: Vec<Pubkey>,
    check_seller_fee_basis_points: bool,
) -> Instruction {
    Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(ruleset, false),
            AccountMeta::new_readonly(authority, true),
            AccountMeta::new(payer, true),
            AccountMeta::new_readonly(solana_program::system_program::id(), false),
        ],
        data: CreatorStandardInstruction::InitRuleset(InitRulesetIx {
            name,
            collector,
            disallowed_addresses,
            allowed_programs,
            check_seller_fee_basis_points,
        })
        .try_to_vec()
        .unwrap(),
    }
}

#[allow(clippy::too_many_arguments)]
pub fn update_ruleset(
    program_id: Pubkey,
    ruleset: Pubkey,
    authority: Pubkey,
    payer: Pubkey,
    collector: Pubkey,
    disallowed_addresses: Vec<Pubkey>,
    allowed_programs: Vec<Pubkey>,
    check_seller_fee_basis_points: bool,
) -> Instruction {
    Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(ruleset, false),
            AccountMeta::new_readonly(authority, true),
            AccountMeta::new(payer, true),
            AccountMeta::new_readonly(solana_program::system_program::id(), false),
        ],
        data: CreatorStandardInstruction::UpdateRuleset(UpdateRulesetIx {
            authority,
            collector,
            disallowed_addresses,
            allowed_programs,
            check_seller_fee_basis_points,
        })
        .try_to_vec()
        .unwrap(),
    }
}
//////// ruleset ////////

//////// mint_manager ////////
#[allow(clippy::too_many_arguments)]
pub fn init_mint_manager(
    program_id: Pubkey,
    mint_manager: Pubkey,
    mint: Pubkey,
    ruleset: Pubkey,
    holder_token_account: Pubkey,
    ruleset_collector: Pubkey,
    collector: Pubkey,
    authority: Pubkey,
    payer: Pubkey,
) -> Instruction {
    Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(mint_manager, false),
            AccountMeta::new(mint, false),
            AccountMeta::new_readonly(ruleset, false),
            AccountMeta::new(holder_token_account, false),
            AccountMeta::new(ruleset_collector, false),
            AccountMeta::new(collector, false),
            AccountMeta::new_readonly(authority, true),
            AccountMeta::new(payer, true),
            AccountMeta::new_readonly(spl_token::id(), false),
            AccountMeta::new_readonly(solana_program::system_program::id(), false),
        ],
        data: CreatorStandardInstruction::InitMintManager
            .try_to_vec()
            .unwrap(),
    }
}

#[allow(clippy::too_many_arguments)]
pub fn update_mint_manager(
    program_id: Pubkey,
    mint_manager: Pubkey,
    ruleset: Pubkey,
    collector: Pubkey,
    authority: Pubkey,
    new_authority: Pubkey,
    payer: Pubkey,
) -> Instruction {
    Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(mint_manager, false),
            AccountMeta::new_readonly(ruleset, false),
            AccountMeta::new(collector, false),
            AccountMeta::new_readonly(authority, true),
            AccountMeta::new(payer, true),
            AccountMeta::new_readonly(solana_program::system_program::id(), false),
        ],
        data: CreatorStandardInstruction::UpdateMintManager(UpdateMintManagerIx {
            authority: new_authority,
        })
        .try_to_vec()
        .unwrap(),
    }
}

#[allow(clippy::too_many_arguments)]
pub fn set_in_use_by(
    program_id: Pubkey,
    mint_manager: Pubkey,
    holder: Pubkey,
    holder_token_account: Pubkey,
    in_use_by_address: Pubkey,
) -> Instruction {
    Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(mint_manager, false),
            AccountMeta::new_readonly(holder, true),
            AccountMeta::new_readonly(holder_token_account, false),
        ],
        data: CreatorStandardInstruction::SetInUseBy(SetInUseByIx { in_use_by_address })
            .try_to_vec()
            .unwrap(),
    }
}

#[allow(clippy::too_many_arguments)]
pub fn remove_in_use_by(program_id: Pubkey, mint_manager: Pubkey, holder: Pubkey) -> Instruction {
    Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(mint_manager, false),
            AccountMeta::new_readonly(holder, true),
        ],
        data: CreatorStandardInstruction::RemoveInUseBy
            .try_to_vec()
            .unwrap(),
    }
}

//////// mint_manager ////////

//////// token ////////
#[allow(clippy::too_many_arguments)]
pub fn approve(
    program_id: Pubkey,
    mint_manager: Pubkey,
    mint: Pubkey,
    holder_token_account: Pubkey,
    holder: Pubkey,
    delegate: Pubkey,
    amount: u64,
) -> Instruction {
    Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new_readonly(mint_manager, false),
            AccountMeta::new_readonly(mint, false),
            AccountMeta::new(holder_token_account, false),
            AccountMeta::new_readonly(holder, true),
            AccountMeta::new_readonly(delegate, false),
            AccountMeta::new_readonly(spl_token::id(), false),
        ],
        data: CreatorStandardInstruction::Approve(ApproveIx { amount })
            .try_to_vec()
            .unwrap(),
    }
}

#[allow(clippy::too_many_arguments)]
pub fn burn(
    program_id: Pubkey,
    mint_manager: Pubkey,
    mint: Pubkey,
    holder_token_account: Pubkey,
    holder: Pubkey,
    delegate: Pubkey,
) -> Instruction {
    Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(mint_manager, false),
            AccountMeta::new(mint, false),
            AccountMeta::new(holder_token_account, false),
            AccountMeta::new(holder, true),
            AccountMeta::new_readonly(delegate, false),
            AccountMeta::new_readonly(spl_token::id(), false),
        ],
        data: CreatorStandardInstruction::Burn.try_to_vec().unwrap(),
    }
}

#[allow(clippy::too_many_arguments)]
pub fn close(
    program_id: Pubkey,
    mint_manager: Pubkey,
    mint: Pubkey,
    token_account: Pubkey,
    owner: Pubkey,
) -> Instruction {
    Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new_readonly(mint_manager, false),
            AccountMeta::new(mint, false),
            AccountMeta::new(token_account, false),
            AccountMeta::new_readonly(owner, true),
            AccountMeta::new_readonly(spl_token::id(), false),
        ],
        data: CreatorStandardInstruction::Close.try_to_vec().unwrap(),
    }
}

#[allow(clippy::too_many_arguments)]
pub fn initialize_account(
    program_id: Pubkey,
    mint: Pubkey,
    token_account: Pubkey,
    owner: Pubkey,
    payer: Pubkey,
    rent: Pubkey,
) -> Instruction {
    Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new_readonly(mint, false),
            AccountMeta::new(token_account, false),
            AccountMeta::new_readonly(owner, false),
            AccountMeta::new(payer, true),
            AccountMeta::new_readonly(rent, false),
            AccountMeta::new_readonly(spl_token::id(), false),
            AccountMeta::new_readonly(spl_associated_token_account::id(), false),
            AccountMeta::new_readonly(solana_program::system_program::id(), false),
        ],
        data: CreatorStandardInstruction::InitializeAccount
            .try_to_vec()
            .unwrap(),
    }
}

#[allow(clippy::too_many_arguments)]
pub fn initialize_mint(
    program_id: Pubkey,
    mint_manager: Pubkey,
    mint: Pubkey,
    ruleset: Pubkey,
    target_token_account: Pubkey,
    target: Pubkey,
    ruleset_collector: Pubkey,
    collector: Pubkey,
    authority: Pubkey,
    payer: Pubkey,
    rent: Pubkey,
) -> Instruction {
    Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(mint_manager, false),
            AccountMeta::new(mint, false),
            AccountMeta::new_readonly(ruleset, false),
            AccountMeta::new(target_token_account, false),
            AccountMeta::new_readonly(target, true),
            AccountMeta::new(ruleset_collector, false),
            AccountMeta::new(collector, false),
            AccountMeta::new_readonly(authority, true),
            AccountMeta::new_readonly(payer, true),
            AccountMeta::new_readonly(rent, false),
            AccountMeta::new_readonly(solana_program::system_program::id(), false),
            AccountMeta::new_readonly(spl_associated_token_account::id(), false),
            AccountMeta::new_readonly(solana_program::system_program::id(), false),
        ],
        data: CreatorStandardInstruction::InitializeMint
            .try_to_vec()
            .unwrap(),
    }
}

#[allow(clippy::too_many_arguments)]
pub fn revoke(
    program_id: Pubkey,
    mint_manager: Pubkey,
    mint: Pubkey,
    holder_token_account: Pubkey,
    holder: Pubkey,
) -> Instruction {
    Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new_readonly(mint_manager, false),
            AccountMeta::new_readonly(mint, false),
            AccountMeta::new(holder_token_account, false),
            AccountMeta::new_readonly(holder, true),
            AccountMeta::new_readonly(spl_token::id(), false),
        ],
        data: CreatorStandardInstruction::Revoke.try_to_vec().unwrap(),
    }
}

#[allow(clippy::too_many_arguments)]
pub fn transfer(
    program_id: Pubkey,
    mint_manager: Pubkey,
    ruleset: Pubkey,
    mint: Pubkey,
    from: Pubkey,
    to: Pubkey,
    authority: Pubkey,
) -> Instruction {
    Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new_readonly(mint_manager, false),
            AccountMeta::new_readonly(ruleset, false),
            AccountMeta::new_readonly(mint, false),
            AccountMeta::new(from, false),
            AccountMeta::new(to, false),
            AccountMeta::new_readonly(authority, true),
            AccountMeta::new_readonly(spl_token::id(), false),
            AccountMeta::new_readonly(solana_program::system_program::id(), false),
            AccountMeta::new_readonly(sysvar::instructions::id(), false),
        ],
        data: CreatorStandardInstruction::Transfer.try_to_vec().unwrap(),
    }
}
//////// token ////////
