use anchor_lang::prelude::*;

use crate::state::product::{Job, STATUS};

use crate::errors::ErrorCode;
// for employee users
pub fn apply_for_job(ctx: Context<ApplyForJobContext>, user_pda: Pubkey) -> Result<()> {
    let job = &mut ctx.accounts.job;

    require!(job.status == STATUS::OPEN, ErrorCode::JobStatusNotOpen);

    job.bidders.push(user_pda);

    Ok(())
}

//for employeR users
pub fn accept_job_application(ctx: Context<UpdateJobContext>, user: Pubkey) -> Result<()> {
    let job = &mut ctx.accounts.job;
    job.user = user;
    job.status = STATUS::INPROGRESS;

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

#[derive(Accounts)]
#[instruction(status: STATUS, user: Pubkey)]
pub struct UpdateJobContext<'info> {
    #[account(mut)]
    owner: Signer<'info>,
    #[account(mut)]
    pub job: Account<'info, Job>,

    pub system_program: Program<'info, System>,
}
