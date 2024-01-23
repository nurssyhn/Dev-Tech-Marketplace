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
        instructions::user::create_employee_profile(ctx, name, email, profile_image, skills)
    }

    pub fn initialize_employer_profile(
        ctx: Context<InitUserProfileContext>,
        name: String,
        email: String,
        profile_image: String,
    ) -> Result<()> {
        instructions::user::create_employer_profile(ctx, name, email, profile_image)
    }

    pub fn initialize_new_job(
        ctx: Context<InitJobContext>,
        id: Pubkey,
        job_title: String,
        job_description: String,
        tags: String,
        amount: u64,
    ) -> Result<()> {
        instructions::job::create_job(ctx, id, job_title, job_description, tags, amount)
    }

    pub fn apply_for_job(ctx: Context<ApplyForJobContext>) -> Result<()> {
        instructions::job::apply_for_job(ctx)
    }

    pub fn accept_job_application(
        ctx: Context<AcceptJobContext>,
        index: u8,
        seed: u64,
    ) -> Result<()> {
        instructions::job::accept_job_application(ctx, index, seed)
    }

    pub fn update_job_completion(ctx: Context<UpdateJobContext>) -> Result<()> {
        instructions::job::update_job_completion(ctx)
    }
}

#[derive(Accounts)]
pub struct Initialize {}
