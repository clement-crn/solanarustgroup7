use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};

declare_id!("892sb2f1GsHR8i5zXYgtKdVTMhSPWLRGLKiVD2Sz1KKp");

#[account]
pub struct Campaign {
    pub creator: Pubkey,
    pub name: String,
    pub target_amount: u64,
    pub current_funds: u64,
}

#[account]
pub struct Donation {
    pub campaign: Pubkey,
    pub donor: Pubkey,
    pub amount: u64,
}

#[account]
pub struct Vote {
    pub campaign: Pubkey,
    pub proposer: Pubkey,
    pub description: String,
    pub yes_votes: u64,
    pub no_votes: u64,
    pub end_time: i64,
}

#[program]
pub mod solanarustgroup7 {
    use super::*;

    pub fn create_campaign(
        ctx: Context<CreateCampaign>,
        name: String,
        target_amount: u64,
    ) -> Result<()> {
        let campaign = &mut ctx.accounts.campaign;

        campaign.creator = *ctx.accounts.creator.key;
        campaign.name = name;
        campaign.target_amount = target_amount;
        campaign.current_funds = 0;

        msg!("Campaign '{}' created by {:?}", campaign.name, campaign.creator);
        Ok(())
    }

    pub fn send_fund(ctx: Context<SendFund>, amount: u64) -> Result<()> {
        let campaign = &mut ctx.accounts.campaign;
        let donation = &mut ctx.accounts.donation;

        require!(amount > 0, CustomError::InvalidAmount);

        campaign.current_funds += amount;

        donation.campaign = campaign.key();
        donation.donor = *ctx.accounts.donor.key;
        donation.amount = amount;

        msg!("Received {} SOL. Total funds: {}", amount, campaign.current_funds);
        Ok(())
    }

    pub fn create_vote(ctx: Context<CreateVote>, description: String, duration: i64) -> Result<()> {
        let vote = &mut ctx.accounts.vote;
        let clock = Clock::get()?;

        vote.campaign = ctx.accounts.campaign.key();
        vote.proposer = *ctx.accounts.proposer.key;
        vote.description = description;
        vote.yes_votes = 0;
        vote.no_votes = 0;
        vote.end_time = clock.unix_timestamp + duration;

        msg!("Vote created for campaign {:?}", vote.campaign);
        Ok(())
    }

    pub fn cast_vote(ctx: Context<CastVote>, support: bool) -> Result<()> {
        let vote = &mut ctx.accounts.vote;
        let clock = Clock::get()?;

        require!(clock.unix_timestamp < vote.end_time, CustomError::VoteEnded);

        if support {
            vote.yes_votes += 1;
        } else {
            vote.no_votes += 1;
        }

        msg!("Vote cast. Yes: {}, No: {}", vote.yes_votes, vote.no_votes);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateCampaign<'info> {
    #[account(init, payer = creator, space = 8 + 32 + (4 + 64) + 8 * 2)]
    pub campaign: Account<'info, Campaign>,
    #[account(mut)]
    pub creator: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SendFund<'info> {
    #[account(mut)]
    pub campaign: Account<'info, Campaign>,
    #[account(init, payer = donor, space = 8 + 32 + 32 + 8)]
    pub donation: Account<'info, Donation>,
    #[account(mut)]
    pub donor: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreateVote<'info> {
    #[account(init, payer = proposer, space = 8 + 32 + 32 + (4 + 128) + 8 * 2)]
    pub vote: Account<'info, Vote>,
    #[account(mut)]
    pub proposer: Signer<'info>,
    #[account(mut)]
    pub campaign: Account<'info, Campaign>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CastVote<'info> {
    #[account(mut)]
    pub vote: Account<'info, Vote>,
    pub voter: Signer<'info>,
    #[account(mut)]
    pub campaign: Account<'info, Campaign>,
}

#[error_code]
pub enum CustomError {
    #[msg("The donation amount must be greater than zero.")]
    InvalidAmount,
    #[msg("The voting period has ended.")]
    VoteEnded,
}
