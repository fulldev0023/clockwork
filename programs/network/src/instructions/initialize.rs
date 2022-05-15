use {
    crate::state::*,
    anchor_lang::{
        prelude::*, 
        solana_program::{instruction::Instruction, system_program, sysvar}
    },
    anchor_spl::token::Mint,
    std::mem::size_of,
};

#[derive(Accounts)]
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
    
    #[account(
        init,
        seeds = [SEED_CYCLER],
        bump,
        payer = admin,
        space = 8 + size_of::<Cycler>(),
    )]
    pub cycler: Account<'info, Cycler>,

    #[account()]
    pub mint: Account<'info, Mint>,

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
    pub system_program: Program<'info, System>
}

pub fn handler<'info>(ctx: Context<'_, '_, '_, 'info, Initialize<'info>>) -> Result<()> {
    // Get accounts
    let admin = &ctx.accounts.admin;
    let authority = &mut ctx.accounts.authority;
    let clock = &ctx.accounts.clock;
    let cycler = &mut ctx.accounts.cycler;
    let config = &mut ctx.accounts.config;
    let mint = &ctx.accounts.mint;
    let registry = &mut ctx.accounts.registry;
    let scheduler_program = &ctx.accounts.scheduler_program;
    let snapshot = &mut ctx.accounts.snapshot;
    let system_program = &ctx.accounts.system_program;

    // Get remaining accounts
    let cycler_task = ctx.remaining_accounts.get(0).unwrap();
    let fee = ctx.remaining_accounts.get(1).unwrap();
    let queue = ctx.remaining_accounts.get(2).unwrap();
    let snapshot_action = ctx.remaining_accounts.get(3).unwrap();
    let snapshot_task = ctx.remaining_accounts.get(4).unwrap();

    // Get bumps
    let authority_bump = *ctx.bumps.get("authority").unwrap();

    // Initialize accounts
    authority.new(queue.key())?;
    config.new(admin.key(),  mint.key())?;
    cycler.new()?;
    registry.new()?;
    registry.new_snapshot(snapshot)?;
    registry.rotate_snapshot(clock, None, snapshot)?;

    // Create a queue
    cronos_scheduler::cpi::queue_new(
        CpiContext::new_with_signer(
            scheduler_program.to_account_info(), 
            cronos_scheduler::cpi::accounts::QueueNew {
                fee: fee.to_account_info(),
                owner: authority.to_account_info(),
                payer: admin.to_account_info(),
                queue: queue.to_account_info(),
                system_program: system_program.to_account_info(),
            },
            &[&[SEED_AUTHORITY, &[authority_bump]]]
        )
    )?;

    // Create a recurring task to cycle the delegate pool
    cronos_scheduler::cpi::task_new(
        CpiContext::new_with_signer(
            scheduler_program.to_account_info(),
            cronos_scheduler::cpi::accounts::TaskNew {
                clock: clock.to_account_info(),
                owner: authority.to_account_info(),
                payer: admin.to_account_info(),
                queue: queue.to_account_info(),
                system_program: system_program.to_account_info(),
                task: cycler_task.to_account_info(),
            },
            &[&[SEED_AUTHORITY, &[authority_bump]]]
        ), 
        "*/20 * * * * * *".into()
    )?;

    // Create a recurring task to take snapshots of the registry
    cronos_scheduler::cpi::task_new(
        CpiContext::new_with_signer(
            scheduler_program.to_account_info(),
            cronos_scheduler::cpi::accounts::TaskNew {
                clock: clock.to_account_info(),
                owner: authority.to_account_info(),
                payer: admin.to_account_info(),
                queue: queue.to_account_info(),
                system_program: system_program.to_account_info(),
                task: snapshot_task.to_account_info(),
            },
            &[&[SEED_AUTHORITY, &[authority_bump]]]
        ), 
        "0 */1 * * * * *".into()
    )?;

    // Add an action to the snapshot task to kick things off
    let next_snapshot_pubkey = Snapshot::pda(1).0;
    let start_snapshot_ix = Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new_readonly(config.key(), false),
            AccountMeta::new(cronos_scheduler::delegate::ID, true),
            AccountMeta::new_readonly(queue.key(), true),
            AccountMeta::new(registry.key(), false),
            AccountMeta::new(next_snapshot_pubkey, false),
            AccountMeta::new_readonly(system_program.key(), false),
        ],
        data: sighash("global", "snapshot_start").into(),
    };
    let rotate_snapshot_ix = Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new_readonly(authority.key(), false),
            AccountMeta::new_readonly(sysvar::clock::ID, false),
            AccountMeta::new_readonly(config.key(), false),
            AccountMeta::new(snapshot.key(), false),
            AccountMeta::new(next_snapshot_pubkey, false),
            AccountMeta::new_readonly(queue.key(), true),
            AccountMeta::new(registry.key(), false),
        ],
        data: sighash("global", "snapshot_rotate").into(),
    };
    cronos_scheduler::cpi::action_new(
        CpiContext::new_with_signer(
            scheduler_program.to_account_info(),
            cronos_scheduler::cpi::accounts::ActionNew {
                action: snapshot_action.to_account_info(),
                owner: authority.to_account_info(),
                payer: admin.to_account_info(),
                queue: queue.to_account_info(),
                system_program: system_program.to_account_info(),
                task: snapshot_task.to_account_info(),
            },
            &[&[SEED_AUTHORITY, &[authority_bump]]],
        ),
        vec![start_snapshot_ix.into(), rotate_snapshot_ix.into()],
    )?;

    Ok(())
}

fn sighash(namespace: &str, name: &str) -> [u8; 8] {
    let preimage = format!("{}:{}", namespace, name);
    let mut sighash = [0u8; 8];
    sighash.copy_from_slice(
        &anchor_lang::solana_program::hash::hash(preimage.as_bytes()).to_bytes()
            [..8],
    );
    sighash
}
