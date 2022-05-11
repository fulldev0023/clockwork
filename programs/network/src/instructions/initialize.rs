use {
    crate::state::*,
    anchor_lang::{
        prelude::*, 
        solana_program::{system_program, sysvar}
    },
    anchor_spl::token::Mint,
    std::mem::size_of,
};

#[derive(Accounts)]
#[instruction(
    // authority_bump: u8,
    // config_bump: u8,
    // fee_bump: u8,
    // pool_bump: u8,
    // queue_bump: u8,
    // registry_bump: u8,
    // snapshot_bump: u8,
    // task_bump: u8,
)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        init,
        seeds = [SEED_AUTHORITY],
        bump,
        payer = admin,
        space = 8 + size_of::<Authority>(),
    )]
    pub authority: Account<'info, Authority>,

    #[account(address = sysvar::clock::ID)]
    pub clock: Sysvar<'info, Clock>,

    #[account(
        init,
        seeds = [SEED_CONFIG],
        bump,
        payer = admin,
        space = 8 + size_of::<Config>(),
    )]
    pub config: Account<'info, Config>,

    #[account(mut)]
    pub fee: AccountInfo<'info>,

    #[account()]
    pub mint: Account<'info, Mint>,

    #[account(
        init,
        seeds = [SEED_POOL],
        bump,
        payer = admin,
        space = 8 + size_of::<Pool>(),
    )]
    pub pool: Account<'info, Pool>,

    #[account(mut)]
    pub queue: AccountInfo<'info>,

    #[account(
        init,
        seeds = [SEED_REGISTRY],
        bump,
        payer = admin,
        space = 8 + size_of::<Registry>(),
    )]
    pub registry: Account<'info, Registry>,
 
    #[account(address = cronos_scheduler::ID)]
    pub scheduler_program: Program<'info, cronos_scheduler::program::CronosScheduler>,

    #[account(
        init,
        seeds = [
            SEED_SNAPSHOT, 
            (0 as u64).to_be_bytes().as_ref()
        ],
        bump,
        payer = admin,
        space = 8 + size_of::<Snapshot>(),
    )]
    pub snapshot: Account<'info, Snapshot>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,

    #[account(mut)]
    pub task: AccountInfo<'info>,
}

pub fn handler(
    ctx: Context<Initialize>,
    // authority_bump: u8,
    // config_bump: u8,
    // fee_bump: u8,
    // pool_bump: u8,
    // queue_bump: u8,
    // registry_bump: u8,
    // snapshot_bump: u8,
    // task_bump: u8,
) -> Result<()> {
    let admin = &ctx.accounts.admin;
    let authority = &mut ctx.accounts.authority;
    let clock = &ctx.accounts.clock;
    let config = &mut ctx.accounts.config;
    // let fee = &mut ctx.accounts.fee;
    let mint = &ctx.accounts.mint;
    let pool = &mut ctx.accounts.pool;
    let queue = &mut ctx.accounts.queue;
    let registry = &mut ctx.accounts.registry;
    // let scheduler_program = &ctx.accounts.scheduler_program;
    let snapshot = &mut ctx.accounts.snapshot;
    // let system_program = &ctx.accounts.system_program;
    let _task = &mut ctx.accounts.task;

    // Get bumps
    msg!("Bumps: {:#?}", ctx.bumps);
    let authority_bump = *ctx.bumps.get("authority").unwrap();
    let config_bump = *ctx.bumps.get("config").unwrap();
    let pool_bump = *ctx.bumps.get("pool").unwrap();
    let registry_bump = *ctx.bumps.get("registry").unwrap();
    let snapshot_bump = *ctx.bumps.get("snapshot").unwrap();

    // Initialize accounts
    authority.new(authority_bump, queue.key())?;
    config.new(admin.key(), config_bump, mint.key())?;
    pool.new(pool_bump)?;
    registry.new(registry_bump)?;
    registry.new_snapshot(snapshot, snapshot_bump)?;
    registry.rotate_snapshot(clock, None, snapshot)?;

    // TODO Make cpi to scheduler program to create a queue
    // cronos_scheduler::cpi::queue_new(
    //     CpiContext::new_with_signer(
    //         scheduler_program.to_account_info(), 
    //         cronos_scheduler::cpi::accounts::QueueNew {
    //             fee: fee.to_account_info(),
    //             owner: authority.to_account_info(),
    //             queue: queue.to_account_info(),
    //             system_program: system_program.to_account_info(),
    //         },
    //         &[&[SEED_AUTHORITY, &[authority_bump]]]
    //     )
    // )?;

    // TODO Make cpi to scheduler program to create a task

    Ok(())
}
