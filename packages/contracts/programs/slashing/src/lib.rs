use anchor_lang::prelude::*;

declare_id!("ADMWFhVnbL6RiudDandcYkhpkZx2FMz8bEx5f97zcS4X");

#[program]
pub mod combined_anchor_program {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let treasury = &mut ctx.accounts.treasury;
        treasury.admin = *ctx.accounts.user.key;
        Ok(())
    }

    pub fn slash(ctx: Context<Slash>, amount: u64) -> Result<()> {
        let treasury = &mut ctx.accounts.treasury;
        Ok(())
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        let treasury = &mut ctx.accounts.treasury;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 32)]
    pub treasury: Account<'info, Treasury>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Slash<'info> {
    #[account(mut)]
    pub treasury: Account<'info, Treasury>,
    pub slasher: Signer<'info>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub treasury: Account<'info, Treasury>,
    pub admin: Signer<'info>,
}

#[account]
pub struct Treasury {
    pub admin: Pubkey,
}
