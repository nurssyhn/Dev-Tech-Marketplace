use anchor_lang::prelude::*;

use crate::{state::apply::ApplyForJob, Job};

pub fn apply_for_job(ctx: Context<ApplyForJobContext>, user_pda: Pubkey) -> Result<()> {
    let job = &mut ctx.accounts.job;
    job.bidders.push(user_pda);

    Ok(())
}

#[derive(Accounts)]
#[instruction(user_pda: Pubkey)]
pub struct ApplyForJobContext<'info> {
    #[account(mut)]
    user: Signer<'info>,
    #[account(mut)]
    pub job: Account<'info, Job>,

    pub system_program: Program<'info, System>,
}
