use anchor_lang::prelude::*;
use std::fmt;
use anchor_lang::solana_program::entrypoint::ProgramResult;
use anchor_lang::prelude::Clock;



declare_id!("9PyH3wLrcfjjCFqMRq5mMapnqs6qb38Vx5XTTRgM3C9Y");

#[program]
pub mod betting_game {
    use super::*;

    pub fn create(ctx: Context<Create>, name: String, description: String,bettingAmount: u64)-> ProgramResult{
        let campaign = &mut ctx.accounts.campaign;
        let clock = Clock::get()?;
        campaign.name = name;
        campaign.description = description;
        campaign.amount_given = 0;
        campaign.admin = *ctx.accounts.user.key;
        campaign.bettingAmount = bettingAmount;
        campaign.creationTime = clock.unix_timestamp;
        campaign.is_campaign = true;
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

    pub fn pay(ctx: Context<Pay>) -> ProgramResult{
        //Check for valid token to pay

        let campaign = &mut ctx.accounts.campaign;
        if campaign.is_campaign{
        let clock = Clock::get()?;
        let currTime = clock.unix_timestamp;
        let mut amount = campaign.bettingAmount;
        if campaign.creationTime - currTime >= 86400 {
            //call the endCampaign and give 10% commission to admin
            return Err(ProgramError::InsufficientFunds);
            }

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
        let mut players = &mut ctx.accounts.campaign.players;
        players.push(ctx.accounts.user.key());
    }
        Ok(())
    }

    pub fn endCampaign(ctx: Context<EndCampaign>) -> ProgramResult{
        let campaign = &mut ctx.accounts.campaign;
        let user = &mut ctx.accounts.user;

        let commissionAmt = campaign.amount_given/10;
        
        if campaign.amount_given>0{
        let ix = anchor_lang::solana_program::system_instruction::transfer(
            &ctx.accounts.campaign.key(),
            &ctx.accounts.campaign.admin.key(),
            commissionAmt
        );

        anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                ctx.accounts.campaign.to_account_info(),
                ctx.accounts.user.to_account_info(),
            ]
        );

    }
    

        Ok(())
    }

    pub fn winner(ctx: Context<Winner>) -> ProgramResult {
        let campaign = &mut ctx.accounts.campaign;

        let winnerOneAmt = campaign.amount_given/2;
        let winnerThreeAmt = campaign.amount_given*3/10;
        let winnerTwoAmt = campaign.amount_given;

        let mut players = &campaign.players;

        if campaign.amount_given>0{
            let ix = anchor_lang::solana_program::system_instruction::transfer(
                &campaign.key(),
                &players[0].key(),
                winnerOneAmt
            );

            // anchor_lang::solana_program::program::invoke(
            //     &ix,
            //     &[
            //         campaign.to_account_info(),
            //         connection.getAccountInfo(new PublicKey(players[0].key())),
    
            //     ]
            // );
    } 
       
    if campaign.amount_given>0{
            let ix = anchor_lang::solana_program::system_instruction::transfer(
                &campaign.key(),
                &players[1].key(),
                winnerTwoAmt
            );
    } 

    if campaign.amount_given>0{
            let ix = anchor_lang::solana_program::system_instruction::transfer(
                &campaign.key(),
                &players[2].key(),
                winnerThreeAmt
            );
    } 

    Ok(())
    }
}

#[derive(Accounts)]
pub struct Create<'info>{
    #[account{init,payer = user, space = 200, seeds=[b"DEMO".as_ref(), user.key().as_ref()],bump}]
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
pub struct EndCampaign<'info>{
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

#[derive(Accounts)]
pub struct Winner<'info>{
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
    pub bettingAmount: u64,
    pub creationTime: i64,
    pub players: Vec<Pubkey>,
    pub is_campaign: bool,
}

// #[error]
// pub enum MyError {
//     #[msg("This is an error message clients cant automatically display")]
//     Hello,
// }
