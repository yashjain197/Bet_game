use anchor_lang::prelude::*;
use std::fmt;
use anchor_lang::solana_program::entrypoint::ProgramResult;


declare_id!("9PyH3wLrcfjjCFqMRq5mMapnqs6qb38Vx5XTTRgM3C9Y");

#[program]
pub mod betting_game {
    use super::*;

    pub fn create(ctx: Context<Create>, name: String, description: String,bettingAmount: u64)-> ProgramResult{
        let campaign = &mut ctx.accounts.campaign;
        campaign.name = name;
        campaign.description = description;
        campaign.amount_given = 0;
        campaign.admin = *ctx.accounts.user.key;
        campaign.bettingAmount = bettingAmount;
        Ok(())
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> ProgramResult{
        let campaign= &mut ctx.accounts.campaign;
        let user = &mut ctx.accounts.user;

        if campaign.admin != *user.key{
            return Err(ProgramError::IncorrectProgramId);
        }

        let rent_balance =  Rent::get()?.minimum_balance(campaign.to_account_info().data_len());

        if **campaign.to_account_info().lamports.borrow() - rent_balance < amount{
            return   Err(ProgramError::InsufficientFunds);
        }

        **campaign.to_account_info().try_borrow_mut_lamports()? -=amount;
        **user.to_account_info().try_borrow_mut_lamports()? +=amount;
        Ok(())
    }

    pub fn pay(ctx: Context<Pay>, amount: u64) -> ProgramResult{
        let ix = anchor_lang::solana_program::system_instruction::transfer(
            &ctx.accounts.user.key(),
            &ctx.accounts.campaign.key(),
            amount
        );
        anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                ctx.accounts.user.to_account_info(),
                ctx.accounts.campaign.to_account_info(),

            ]
        );
        (&mut ctx.accounts.campaign).amount_given += amount;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Create<'info>{
    #[account{init,payer = user, space = 9000, seeds=[b"DEMO".as_ref(), user.key().as_ref()],bump}]
    pub campaign: Account<'info, Campaign>,

    #[account{mut}]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct Withdraw<'info>{
    #[account(mut)]
    pub campaign: Account<'info, Campaign>,

    #[account(mut)]
    pub user: Signer<'info>
}

#[derive(Accounts)]
pub struct Pay<'info>{
    #[account(mut)]
    pub campaign: Account<'info, Campaign>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>
}


#[account]
pub struct Campaign{
    pub admin: Pubkey,
    pub name: String,
    pub description: String,
    pub amount_given: u64,
    pub bettingAmount: u64
}
