use anchor_lang::{
    solana_program::{
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
    },
    InstructionData,
};

pub fn queue_cancel(manager: Pubkey, queue: Pubkey, owner: Pubkey) -> Instruction {
    Instruction {
        program_id: cronos_scheduler::ID,
        accounts: vec![
            AccountMeta::new(owner, true),
            AccountMeta::new_readonly(manager, false),
            AccountMeta::new(queue, false),
        ],
        data: cronos_scheduler::instruction::QueueCancel {}.data(),
    }
}
