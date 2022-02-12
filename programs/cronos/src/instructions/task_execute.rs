use {
    crate::{errors::*, state::*},
    anchor_lang::prelude::*,
    solana_program::{instruction::Instruction, program::invoke_signed, sysvar},
};

#[derive(Accounts)]
#[instruction()]
pub struct TaskExecute<'info> {
    #[account(address = sysvar::clock::ID)]
    pub clock: Sysvar<'info, Clock>,

    #[account(
        seeds = [SEED_CONFIG],
        bump = config.bump,
        owner = crate::ID
    )]
    pub config: Account<'info, Config>,

    #[account(
        mut,
        seeds = [
            SEED_DAEMON, 
            daemon.owner.key().as_ref()
        ],
        bump = daemon.bump,
        owner = crate::ID
    )]
    pub daemon: Account<'info, Daemon>,

    #[account(
        mut,
        seeds = [
            SEED_FEE,
            fee.daemon.as_ref()
        ],
        bump = fee.bump,
        constraint = fee.daemon == daemon.key(),
        owner = crate::ID
    )]
    pub fee: Account<'info, Fee>,

    #[account(
        mut,
        seeds = [
            SEED_TASK, 
            task.daemon.as_ref(),
            task.int.to_be_bytes().as_ref(),
        ],
        bump = task.bump,
        has_one = daemon,
        constraint = task.status == TaskStatus::Queued @ ErrorCode::TaskNotPending,
        constraint = task.exec_at <= clock.unix_timestamp @ ErrorCode::TaskNotDue,
        owner = crate::ID
    )]
    pub task: Account<'info, Task>,

    #[account(mut)]
    pub worker: Signer<'info>,
}

pub fn handler(ctx: Context<TaskExecute>) -> ProgramResult {
    // Get accounts.
    let config = &ctx.accounts.config;
    let daemon = &mut ctx.accounts.daemon;
    let fee = &mut ctx.accounts.fee;
    let task = &mut ctx.accounts.task;
    let worker = &mut ctx.accounts.worker;

    // Update task state.
    let next_exec_at = task.exec_at.checked_add(task.interval).unwrap();
    if task.interval == 0 || next_exec_at >= task.stop_at {
        task.status = TaskStatus::Done;
    } else {
        task.exec_at = next_exec_at;
    }

    // Increment collectable fee balance. 
    fee.balance = fee.balance.checked_add(config.program_fee).unwrap();

    // Invoke instruction.
    invoke_signed(
        &Instruction::from(&task.ix),
        &ctx.remaining_accounts.iter().as_slice(),
        &[&[SEED_DAEMON, daemon.owner.key().as_ref(), &[daemon.bump]]],
    )?;

    // Transfer lamports from daemon to fee account.
    **daemon.to_account_info().try_borrow_mut_lamports()? -= config.program_fee;
    **fee.to_account_info().try_borrow_mut_lamports()? += config.program_fee;

    // Transfer lamports from daemon to worker.
    **daemon.to_account_info().try_borrow_mut_lamports()? -= config.worker_fee;
    **worker.to_account_info().try_borrow_mut_lamports()? += config.worker_fee;

    Ok(())
}
