use anchor_lang::prelude::*;

declare_id!("");

#[program]
pub mod reward_manager {
    use super::*;

    // Initializing the RewardTreasury Account
    pub fn initialize_treasury(ctx: Context<InitializeTreasury>, admin_address: Pubkey) -> Result<()> {
        let treasury_account = &mut ctx.accounts.treasury;
        treasury_account.admin_address = admin_address;
        Ok(())
    }

    // Creating a new user instance
    pub fn new_user_instance(
        ctx: Context<CreateUserInstance>,
        client_addresses: Vec<Pubkey>,
        number_of_epochs: u64,
    ) -> Result<()> {
        let user_instance = &mut ctx.accounts.user_instance;
        user_instance.client_addresses = client_addresses;
        user_instance.number_of_epochs = number_of_epochs;
        Ok(())
    }

    // Collect training fees and distribute them to clients
    pub fn collect_training_fees(ctx: Context<CollectTrainingFees>) -> Result<()> {
        let user_instance = &ctx.accounts.user_instance;
        let fee_per_epoch = ctx.accounts.reward_treasury.fee_per_epoch;
        let protocol_fee_percentage = ctx.accounts.reward_treasury.protocol_fee_percentage;
        let number_of_epochs = user_instance.number_of_epochs;

        let total_fee = number_of_epochs.checked_mul(fee_per_epoch).unwrap();
        if ctx.accounts.payer.lamports() < total_fee {
            return Err(ErrorCode::NotEnoughFunds.into());
        }

        let protocol_fee = total_fee.checked_mul(protocol_fee_percentage as u64).unwrap() / 100;
        let client_fee = (total_fee - protocol_fee) / (user_instance.client_addresses.len() as u64);

        // Transfer fees to clients
        for client_address in &user_instance.client_addresses {
            let client_account = AccountInfo::try_from(client_address)?;
            invoke(
                &solana_program::system_instruction::transfer(&ctx.accounts.payer.key(), &client_account.key, client_fee),
                &[
                    ctx.accounts.payer.to_account_info(),
                    client_account.to_account_info(),
                    ctx.accounts.system_program.to_account_info(),
                ],
            )?;
        }

        // Transfer protocol fee to treasury
        invoke(
            &solana_program::system_instruction::transfer(&ctx.accounts.payer.key(), &ctx.accounts.reward_treasury.to_account_info().key(), protocol_fee),
            &[
                ctx.accounts.payer.to_account_info(),
                ctx.accounts.reward_treasury.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
        )?;

        Ok(())
    }

    // Withdraw funds from the treasury
    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        let treasury = &mut ctx.accounts.treasury;
        if ctx.accounts.admin.key != treasury.admin_address {
            return Err(ErrorCode::NotAdmin.into());
        }
        if treasury.to_account_info().lamports() < amount {
            return Err(ErrorCode::NotEnoughBalance.into());
        }
        **treasury.to_account_info().lamports.borrow_mut() -= amount;
        **ctx.accounts.admin.lamports.borrow_mut() += amount;
        Ok(())
    }
}

#[account]
pub struct RewardTreasury {
    pub admin_address: Pubkey,
    pub fee_per_epoch: u64,
    pub protocol_fee_percentage: u8, // stored as a percentage (e.g., 10 = 10%)
}

#[account]
pub struct UserInstance {
    pub client_addresses: Vec<Pubkey>,
    pub number_of_epochs: u64,
}

#[derive(Accounts)]
pub struct InitializeTreasury<'info> {
    #[account(init, payer = admin, space = 8 + 32 + 8 + 1)]
    pub treasury: Account<'info, RewardTreasury>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreateUserInstance<'info> {
    #[account(init, payer = admin, space = 8 + 32 * 10 + 8)]
    pub user_instance: Account<'info, UserInstance>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CollectTrainingFees<'info> {
    #[account(mut)]
    pub user_instance: Account<'info, UserInstance>,
    #[account(mut)]
    pub reward_treasury: Account<'info, RewardTreasury>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub treasury: Account<'info, RewardTreasury>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Not enough funds to collect the training fees.")]
    NotEnoughFunds,
    #[msg("Not enough balance in the treasury.")]
    NotEnoughBalance,
    #[msg("You are not authorized to perform this action.")]
    NotAdmin,
}
