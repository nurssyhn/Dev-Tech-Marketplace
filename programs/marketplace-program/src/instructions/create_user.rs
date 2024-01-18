use anchor_lang::prelude::*;

use crate::state::user::UserProfile;
// Add this line to import the `state` module

pub fn create_employee_profile(
    ctx: Context<InitUserProfileContext>,
    name: String,
    email: String,
    profile_image: String,
    skills: Option<Vec<String>>,
) -> Result<()> {
    // add a check if pubkey already exists :: respond with error
    //

    //create user profile
    UserProfile::init_employee_profile(
        &mut ctx.accounts.user_profile,
        name,
        email,
        profile_image,
        skills,
    )?;

    Ok(())
}

#[derive(Accounts)]
#[instruction(name: String, email: String, profile_image: String, skills: Option<Vec<String>>,)]
pub struct InitUserProfileContext<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        init,
        payer = user, 
        seeds=[b"User", user.key().as_ref()], 
        bump, 
        space = 8 + UserProfile::MAX_SIZE,
    )]
    pub user_profile: Account<'info, UserProfile>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(name: String, email: String, profile_image: String, skills: Option<Vec<String>>,)]

pub struct UpdateUserProfile<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        init_if_needed,
        payer = user, 
        seeds=[b"User", user.key().as_ref()], 
        bump, 
        space = 8 + UserProfile::MAX_SIZE,
    )]
    pub user_profile: Account<'info, UserProfile>,

    pub system_program: Program<'info, System>,
}
