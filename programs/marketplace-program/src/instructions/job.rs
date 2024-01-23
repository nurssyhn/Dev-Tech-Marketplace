use anchor_lang::prelude::*;
use anchor_lang::system_program::{Transfer, transfer};
use anchor_spl::token_interface::spl_token_2022::solana_zk_token_sdk::zk_token_proof_instruction::transfer;

use crate::state::job::Job;
use crate::{STATUS, Escrow, escrow};

use crate::errors::ErrorCode;
//for employeR users


pub fn create_job(
    ctx: Context<InitJobContext>,
    id: Pubkey,
    job_title: String,
    job_description: String,
    tags: String,
    amount: u64,

) -> Result<()> {


   Job::init_job(
    &mut ctx.accounts.job,
    id,
    ctx.accounts.owner.key(),
    job_title,
    job_description,
    tags,
    amount,
   )?;

   

    Ok(())
}


pub fn accept_job_application(ctx: Context<AcceptJobContext>, index: u8,seed:u64) -> Result<()> {
    let job = &mut ctx.accounts.job;
    require!(job.owner == ctx.accounts.owner.key(), ErrorCode::UserNotAuthorized);
    job.user = job.bidders[index as usize];


    let escrow = &mut ctx.accounts.escrow;
    if !escrow.is_initialized {
        Escrow::init_escrow(
            escrow,
            seed,
            ctx.accounts.owner.key(),
            job.user,
            job.id,
            job.amount,
            true
        )?;
    }

    let transfer_accounts = Transfer {
        from: ctx.accounts.owner.to_account_info(),
        to: ctx.accounts.escrow.to_account_info(),
    };

    let cpi_ctx = CpiContext::new(ctx.accounts.system_program.to_account_info(), transfer_accounts);

    transfer(cpi_ctx, job.amount)?;

    job.status = STATUS::INPROGRESS;

    Ok(())
}


// for employee users
pub fn apply_for_job(ctx: Context<ApplyForJobContext>) -> Result<()> {
    let job = &mut ctx.accounts.job;

    require!(job.status == STATUS::OPEN, ErrorCode::JobStatusNotOpen);

    job.bidders.push(ctx.accounts.user.key());

    Ok(())
}

pub fn update_job_completion(ctx: Context<UpdateJobContext>) -> Result<()> {
    let job = &mut ctx.accounts.job;

    require!(
        job.owner == ctx.accounts.owner.key(),
    ErrorCode::UserNotAuthorized
    );
    job.status = STATUS::COMPLETED;

    let transfer_accounts = Transfer {
        from: ctx.accounts.vault.to_account_info(),
        to: ctx.accounts.user.to_account_info(),
    };

    let cpi_ctx = CpiContext::new(ctx.accounts.system_program.to_account_info(), transfer_accounts);

    transfer(cpi_ctx, ctx.accounts.job.amount)?;

    Ok(())
}



#[derive(Accounts)]
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



#[derive(Accounts)]
pub struct ApplyForJobContext<'info> {
    #[account(mut)]
    user: Signer<'info>,
    #[account(mut)]
    pub job: Account<'info, Job>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(seed:u64)]
pub struct AcceptJobContext<'info> {
    #[account(mut)]
    owner: Signer<'info>,
    #[account(mut)]
    pub job: Account<'info, Job>,
    #[account(
        init,
        payer= owner,
        space= 8 + Escrow::MAX_SIZE,
        seeds= [b"escrow", owner.key().as_ref(),seed.to_le_bytes().as_ref()],
        bump
       
    )]
    pub escrow: Box<Account<'info, Escrow>>,
    /// CHECK : The token vault to deposit the funds into.
    #[account(
        seeds=[b"vault", escrow.key().as_ref(),seed.to_le_bytes().as_ref()],
        bump
    )]
    pub vault: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
}


#[derive(Accounts)]
#[instruction(seed:u64)]
pub struct UpdateJobContext<'info> {
    #[account(mut)]
    owner: Signer<'info>,
    #[account(mut)]
    user: SystemAccount<'info>,
    #[account(mut)]
    pub job: Account<'info, Job>,
    #[account(
        seeds= [b"escrow", owner.key().as_ref(),seed.to_le_bytes().as_ref()],
        bump
       
    )]
    pub escrow: Box<Account<'info, Escrow>>,
    #[account(
        mut,
        seeds=[b"vault", escrow.key().as_ref(),seed.to_le_bytes().as_ref()],
        bump,
    )]
    pub vault: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}