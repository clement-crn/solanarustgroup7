use anchor_lang::prelude::*;

declare_id!("892sb2f1GsHR8i5zXYgtKdVTMhSPWLRGLKiVD2Sz1KKp");

#[program]
pub mod solanarustgroup7 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
