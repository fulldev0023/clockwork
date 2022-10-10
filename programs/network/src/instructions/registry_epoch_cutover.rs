use clockwork_utils::CrankResponse;

use {crate::objects::*, anchor_lang::prelude::*};

#[derive(Accounts)]
pub struct RegistryEpochCutover<'info> {
    #[account(address = Config::pubkey())]
    pub config: Account<'info, Config>,

    #[account(address = config.epoch_queue)]
    pub queue: Signer<'info>,

    #[account(
        mut,
        seeds = [SEED_REGISTRY],
        bump,
    )]
    pub registry: Account<'info, Registry>,
}

pub fn handler(ctx: Context<RegistryEpochCutover>) -> Result<CrankResponse> {
    // Get accounts.
    let registry = &mut ctx.accounts.registry;

    // Move the current epoch forward.
    registry.current_epoch = registry.current_epoch.checked_add(1).unwrap();
    registry.locked = false;

    // TODO Build next instruction for queue.
    // TODO (optional) For cost-efficiency, close the snapshot accounts and return the lamports to a queue.

    Ok(CrankResponse {
        next_instruction: None,
        ..CrankResponse::default()
    })
}