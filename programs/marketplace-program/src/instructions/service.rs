use anchor_lang::prelude::*;

use crate::state::service::Service;
use crate::STATUS;

use crate::errors::ErrorCode;

pub fn create_service(
    ctx: Context<InitServiceContext>,
    id: Pubkey,
    service_title: String,
    service_description: String,
    tags: String,
    amount: u64,
) -> Result<()> {


   Service::init_service(
    &mut ctx.accounts.service,
    id,
    ctx.accounts.owner.key(),
    service_title,
    service_description,
    tags,
    amount,
   )?;

   

    Ok(())
}


pub fn update_service_completion(ctx: Context<UpdateServiceContext>) -> Result<()> {
    let service = &mut ctx.accounts.service;

    require!(
        service.user == ctx.accounts.owner.key(),
    ErrorCode::UserNotAuthorized
    );
    service.status = STATUS::COMPLETED;

    Ok(())
}




// for employee users
pub fn apply_for_service(ctx: Context<ApplyForServiceContext>) -> Result<()> {
    let service = &mut ctx.accounts.service;

    require!(service.status == STATUS::OPEN, ErrorCode::ServiceStatusNotOpen);

    // create a service account with the user as the owner


    Ok(())
}



#[derive(Accounts)]
pub struct InitServiceContext<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(
        init,
        payer = owner, 
        seeds=[b"Service", owner.key().as_ref()], 
        bump, 
        space = 8 + Service::MAX_SIZE,
    )]
    pub service: Account<'info, Service>,

    pub system_program: Program<'info, System>,
}



#[derive(Accounts)]
pub struct ApplyForServiceContext<'info> {
    #[account(mut)]
    owner: Signer<'info>,
    #[account(mut)]
    pub service: Account<'info, Service>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateServiceContext<'info> {
    #[account(mut)]
    owner: Signer<'info>,
    #[account(mut)]
    pub service: Account<'info, Service>,

    pub system_program: Program<'info, System>,
}
