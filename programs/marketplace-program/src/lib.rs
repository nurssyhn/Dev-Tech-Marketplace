use anchor_lang::prelude::*;

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
}

#[derive(Accounts)]
pub struct Initialize {}
