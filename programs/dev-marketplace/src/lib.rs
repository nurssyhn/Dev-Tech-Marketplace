use anchor_lang::prelude::*;

declare_id!("FYtx9JPLrd3NYu3GkMbfNVPV1jJeNU3DUxx4mBjDwZsx");

#[program]
pub mod dev_marketplace {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
