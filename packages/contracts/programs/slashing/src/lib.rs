use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_program;

declare_id!("6QNt2egeDnNuX8peia8QXEapztQSv6f5EV3MHgJ2mcmD");

#[program]
pub mod slashing_manager {
    use super::*;

    // Initialize program with admin and slasher
    pub fn initialize(ctx: Context<Initialize>, admin: Pubkey, slasher: Pubkey) -> Result<()> {
        let state = &mut ctx.accounts.state;
        state.admin = admin;
        state.slasher = slasher;
        Ok(())
    }

    // Slash client stakes
    pub fn slash_clients(
        ctx: Context<SlashClients>,
        instance_id: u64,
        client_addresses: Vec<Pubkey>,
    ) -> Result<()> {
        let state = &ctx.accounts.state;

        // Check if the caller is the authorized slasher
        require!(state.slasher == *ctx.accounts.slasher.key, CustomError::UnauthorizedSlasher);

        // Perform slashing logic (this would interact with other programs)
        // Placeholder: For each client address, call the staking registry to slash their stake
        for client_address in client_addresses {
            // Call another program to slash the client's stake (not implemented here)
            // staking_registry::cpi::slash(ctx.accounts.staking_registry.to_account_info(), client_address)?;
        }

        // Emit event (in Anchor, events are logged via logs)
        msg!("Slashing clients for instance ID: {:?}", instance_id);

        Ok(())
    }

    // Withdraw from the treasury
    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        let state = &ctx.accounts.state;

        // Check if the caller is the admin
        require!(state.admin == *ctx.accounts.admin.key, CustomError::UnauthorizedAdmin);

        // Check if there's enough balance
        let treasury_balance = **ctx.accounts.treasury_account.to_account_info().lamports.borrow();
        require!(treasury_balance >= amount, CustomError::InsufficientBalance);

        // Transfer the funds
        **ctx.accounts.treasury_account.to_account_info().try_borrow_mut_lamports()? -= amount;
        **ctx.accounts.admin.to_account_info().try_borrow_mut_lamports()? += amount;

        Ok(())
    }
}

// Define the accounts structure
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = admin, space = 8 + 32 + 32)]
    pub state: Account<'info, State>,
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(mut)]
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SlashClients<'info> {
    #[account(mut)]
    pub state: Account<'info, State>,
    pub slasher: Signer<'info>,
    // Account of the staking registry (interface to call slashing)
    // pub staking_registry: AccountInfo<'info>,  // Placeholder for future staking registry logic
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub state: Account<'info, State>,
    #[account(mut)]
    pub treasury_account: SystemAccount<'info>, // Solana's native treasury account
    #[account(mut)]
    pub admin: Signer<'info>,
}

// Define the state for the contract
#[account]
pub struct State {
    pub admin: Pubkey,   // Address of the admin
    pub slasher: Pubkey, // Address of the slasher
}

// Custom errors for unauthorized actions or insufficient balance
#[error_code]
pub enum CustomError {
    #[msg("Unauthorized slasher")]
    UnauthorizedSlasher,
    #[msg("Unauthorized admin")]
    UnauthorizedAdmin,
    #[msg("Insufficient balance in treasury")]
    InsufficientBalance,
}
