use anchor_lang::prelude::*;

pub mod context;
pub use context::*;
pub mod state;  
pub use state::*;
pub mod errors;
pub use errors::*;

declare_id!("FYtx9JPLrd3NYu3GkMbfNVPV1jJeNU3DUxx4mBjDwZsx");


#[program]
pub mod DevMarketplace{
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
