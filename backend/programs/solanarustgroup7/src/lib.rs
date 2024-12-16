use anchor_lang::prelude::*;

declare_id!("892sb2f1GsHR8i5zXYgtKdVTMhSPWLRGLKiVD2Sz1KKp");
#[account]
pub struct Campaign {
    pub creator: Pubkey,      
    pub name: String,          
    pub description: String,   
    pub target_amount: u64,    
    pub current_funds: u64,    
}
// ------------CREATE CAMPAIGN---------
#[program]
pub mod solanarustgroup7 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }


    pub fn create_campaign(
        ctx: Context<CreateCampaign>,
        name: String,
        description: String,
        target_amount: u64,
    ) -> Result<()> {
        let campaign = &mut ctx.accounts.campaign;

        campaign.creator = *ctx.accounts.creator.key;
        campaign.name = name;
        campaign.description = description;
        campaign.target_amount = target_amount;
        campaign.current_funds = 0;

        msg!(
            "Campaign '{}' created by {:?} with target {}",
            campaign.name,
            campaign.creator,
            campaign.target_amount
        );

        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateCampaign<'info> {
    #[account(
        init,
        payer = creator,
        space = 8 + 32 + (4 + 64) * 2 + 8 * 2 
    )]
    pub campaign: Account<'info, Campaign>,
    #[account(mut)]
    pub creator: Signer<'info>,
    pub system_program: Program<'info, System>,
}


#[derive(Accounts)]
pub struct Initialize {}

//------------SEND FUND FROM FAKE ANOTHER ACCOUNT---------
// DEV IN PROGRESS /!\
#[program]
pub mod solanarustgroup7 {
    use super::*;

    pub fn send_fund(ctx: Context<SendFund>, amount: u64) -> Result<()> {
        let campaign = &mut ctx.accounts.campaign;

        campaign.current_funds += amount;

        msg!(
            "Received {} SOL from {:?}. Total funds: {}",
            amount,
            ctx.accounts.from.key,
            campaign.current_funds
        );

        Ok(())
    }
}

#[derive(Accounts)]
pub struct SendFund<'info> {
    #[account(mut)]
    pub campaign: Account<'info, Campaign>,
    pub from: AccountInfo<'info>,
}





