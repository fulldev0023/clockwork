use {
    crate::state::*,
    anchor_lang::{prelude::*, solana_program::system_program},
    std::mem::size_of
};

#[derive(Accounts)]
#[instruction(ixs: Vec<InstructionData>)]
pub struct TaskNew<'info> {
    #[account(
        init,
        seeds = [
            SEED_TASK, 
            queue.key().as_ref(),
            queue.task_count.to_be_bytes().as_ref(),
        ],
        bump,
        payer = payer,
        space = 8 + size_of::<Task>() + borsh::to_vec(&ixs).unwrap().len(),
    )]
    pub task: Account<'info, Task>,
    
    #[account()]
    pub owner: Signer<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        seeds = [
            SEED_YOGI, 
            yogi.owner.as_ref()
        ],
        bump,
        has_one = owner,
    )]
    pub yogi: Account<'info, Yogi>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,

    #[account(
        mut,
        seeds = [
            SEED_QUEUE, 
            yogi.key().as_ref(),
            queue.id.to_be_bytes().as_ref(),
        ],
        bump,
        has_one = yogi,
    )]
    pub queue: Account<'info, Queue>,
}

pub fn handler(
    ctx: Context<TaskNew>,
    ixs: Vec<InstructionData>,
) -> Result<()> {
    let task = &mut ctx.accounts.task;
    let queue = &mut ctx.accounts.queue;

    task.new( ixs, queue)
}
