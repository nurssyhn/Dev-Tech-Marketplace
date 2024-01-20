use anchor_lang::prelude::*;
pub mod errors;
pub mod instructions;
use instructions::*;

pub mod state;

use state::*;
declare_id!("Hnf7nuJLhXP5pMh7AQrqratPUWcAXzsw6MUf6vacY23V");

#[program]
pub mod marketplace_program {
    use super::*;

    pub fn initialize_employee_profile(
        ctx: Context<InitUserProfileContext>,
        name: String,
        email: String,
        profile_image: String,
        skills: Option<Vec<String>>,
    ) -> Result<()> {
        instructions::create_user::create_employee_profile(ctx, name, email, profile_image, skills)
    }

    pub fn initialize_employer_profile(
        ctx: Context<InitUserProfileContext>,
        name: String,
        email: String,
        profile_image: String,
    ) -> Result<()> {
        instructions::create_user::create_employer_profile(ctx, name, email, profile_image)
    }

    pub fn initialize_new_job(
        ctx: Context<InitJobContext>,
        id: Pubkey,
        owner: Pubkey,
        job_title: String,
        job_description: String,
        tags: String,
        amount: u64,
    ) -> Result<()> {
        instructions::create_product::create_job(
            ctx,
            id,
            owner,
            job_title,
            job_description,
            tags,
            amount,
        )
    }

    pub fn apply_for_job(ctx: Context<ApplyForJobContext>, user_pda: Pubkey) -> Result<()> {
        instructions::job::apply_for_job(ctx, user_pda)
    }

    pub fn accept_job_application(ctx: Context<UpdateJobContext>, user: Pubkey) -> Result<()> {
        instructions::job::accept_job_application(ctx, user)
    }
}

#[derive(Accounts)]
pub struct Initialize {}
