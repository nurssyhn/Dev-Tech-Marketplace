use anchor_lang::prelude::*;

#[account]
pub struct PlatformListing {
    jobs: Vec<Pubkey>,
    services: Vec<Pubkey>,
}

impl PlatformListing {
    pub const MAX_SIZE: usize = 4 + (100 * 32) //jobs
    + 4 + (100 * 32); //services
}
