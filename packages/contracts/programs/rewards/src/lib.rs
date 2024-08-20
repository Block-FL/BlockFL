use anchor_lang::prelude::*;

declare_id!("A35dumYpPgWwUfYYed3FUeFFVHHB7zTugSZM1ho7EPXV");

#[program]
pub mod rewards {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, config: Vec<u8>) -> Result<()> {
        let reward_manager = &mut ctx.accounts.reward_manager;

        reward_manager.some_field = config[0];

        Ok(())
    }

}

#[account]
pub struct RewardManager {
    pub some_field: u8, 
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 8)] 
    pub reward_manager: Account<'info, RewardManager>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
