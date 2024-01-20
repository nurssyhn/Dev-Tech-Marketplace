use anchor_lang::prelude::*;
#[account]
pub struct ApplyForJob {
    pub job_id: Pubkey,
}

impl ApplyForJob {
    pub const MAX_SIZE: usize = 32;
}

#[account]
pub struct ApplyForService {
    pub service_id: Pubkey,
}

impl ApplyForService {
    pub const MAX_SIZE: usize = 32;
}
