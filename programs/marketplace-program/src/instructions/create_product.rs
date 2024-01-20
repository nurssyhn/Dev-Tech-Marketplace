use anchor_lang::prelude::*;

use crate::state::product::Job;
// Add this line to import the `state` module

pub fn create_job(
    ctx: Context<InitJobContext>,
    id: Pubkey,
    owner: Pubkey,
    job_title: String,
    job_description: String,
    tags: String,
    amount: u64,

) -> Result<()> {


   
   Job::init_job(
    &mut ctx.accounts.job,
    id,
    owner,
    job_title,
    job_description,
    tags,
    amount,
   )?;

   

    Ok(())
}

#[derive(Accounts)]
#[instruction(id: Pubkey, owner: Pubkey, job_title: String, job_description: String, tags: String, amount: u64)]
pub struct InitJobContext<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(
        init,
        payer = owner, 
        seeds=[b"Job", owner.key().as_ref()], 
        bump, 
        space = 8 + Job::MAX_SIZE,
    )]
    pub job: Account<'info, Job>,

    pub system_program: Program<'info, System>,
}

