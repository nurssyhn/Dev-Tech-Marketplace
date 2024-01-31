use anchor_lang::prelude::*;
use anchor_lang::system_program::{Transfer, transfer};

use crate::state::service::Service;
use crate::{Escrow, STATUS};

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



pub fn apply_for_service(ctx: Context<ApplyForServiceContext>,seed:u64) -> Result<()> {
    let service = &mut ctx.accounts.service;

    require!(service.owner == ctx.accounts.owner.key(), ErrorCode::UserNotAuthorized);
    require!(service.service_status == STATUS::OPEN, ErrorCode::ServiceStatusNotOpen);

    let escrow = &mut ctx.accounts.escrow;
    if !escrow.is_initialized {
        Escrow::init_escrow(
            escrow,
            seed,
            ctx.accounts.user.key(),
            ctx.accounts.owner.key(),
            service.id,
            service.amount,
            true
        )?;
    }

    let transfer_accounts = Transfer {
        from: ctx.accounts.user.to_account_info(),
        to: ctx.accounts.vault.to_account_info(),
    };

    let cpi_ctx = CpiContext::new(ctx.accounts.system_program.to_account_info(), transfer_accounts);

    transfer(cpi_ctx, service.amount)?;

    // create a service account with the user as the owner
    let _ = Service::buy_service( &mut ctx.accounts.service_pda, 
        service.id, 
        service.owner, 
        ctx.accounts.user.key(), 
        service.service_title.clone(), 
        service.service_description.clone(), 
        service.tags.clone(), 
        service.amount);


    Ok(())
}


pub fn update_service_completion(ctx: Context<UpdateServiceContext>) -> Result<()> {
    let service = &mut ctx.accounts.service;

    require!(
        service.owner == ctx.accounts.owner.key(),
    ErrorCode::UserNotAuthorized
    );
    service.service_status = STATUS::COMPLETED;

    Ok(())
}

pub fn update_service_payment(ctx: Context<UpdateServicePaymentContext>, bumps: u8) -> Result<()> {
    let service = &mut ctx.accounts.service;
    require!(service.service_status == STATUS::COMPLETED, ErrorCode::InvalidServiceStatus);
    let seeds = &[
                    &b"servicevault"[..],
                    &ctx.accounts.escrow.key().to_bytes()[..], 
                    &[bumps]
                ];
                let signer_seeds = &[&seeds[..]];
                require!(
                    service.user == ctx.accounts.user.key(),
                ErrorCode::UserNotAuthorized
                );
    require!(
        service.owner == ctx.accounts.owner.key(),
    ErrorCode::UserNotAuthorized
    );
   

    let transfer_accounts = Transfer {
        from: ctx.accounts.vault.to_account_info(),
        to: ctx.accounts.owner.to_account_info(),
    };

    let cpi_ctx = CpiContext::new_with_signer(ctx.accounts.system_program.to_account_info(), transfer_accounts, signer_seeds);

    transfer(cpi_ctx, service.amount)?;
    service.payment_status = STATUS::PAID;

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
    owner: SystemAccount<'info>,
    #[account(mut)]
    user: Signer<'info>,
    #[account(mut)]
    pub service: Account<'info, Service>,
    #[account(
        init,
        payer = user, 
        seeds=[b"Service", user.key().as_ref()], 
        bump, 
        space = 8 + Service::MAX_SIZE,
    )]
    pub service_pda: Account<'info, Service>,
    #[account(
        init,
        payer= user,
        space= 8 + Escrow::MAX_SIZE,
        seeds= [b"serviceescrow", user.key().as_ref()],
        bump
       
    )]
    pub escrow: Box<Account<'info, Escrow>>,
    /// CHECK : The token vault to deposit the funds into.
    #[account(
        mut,
        seeds=[b"servicevault", escrow.key().as_ref()],
        bump
    )]
    pub vault: UncheckedAccount<'info>,

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


#[derive(Accounts)]
pub struct UpdateServicePaymentContext<'info> {
    #[account(mut)]
    user: Signer<'info>,
    #[account(mut)]
    owner: SystemAccount<'info>,
    #[account(mut)]
    pub service: Account<'info, Service>,
    
#[account(
    mut,
    seeds= [b"serviceescrow", user.key().as_ref()],
    bump
   
)]
pub escrow: Box<Account<'info, Escrow>>,
#[account(
    mut,
    seeds=[b"servicevault", escrow.key().as_ref()],
    bump,
)]
pub vault: SystemAccount<'info>,
    

    pub system_program: Program<'info, System>,
}


