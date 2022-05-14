use {
    crate::state::*,
    anchor_lang::{prelude::*, solana_program::system_program},
    std::mem::size_of,
};

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

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
        seeds = [SEED_HEARTBEAT],
        bump,
        payer = admin,
        space = 8 + size_of::<Heartbeat>(),
    )]
    pub heartbeat: Account<'info, Heartbeat>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<Initialize>) -> Result<()> {
    let admin = &ctx.accounts.admin;
    let config = &mut ctx.accounts.config;
    let heartbeat = &mut ctx.accounts.heartbeat;

    config.new(admin.key())?;
    heartbeat.new()?;

    Ok(())
}
