use anchor_lang::prelude::*;

// Struct for job details
#[account]
pub struct JobDetails {
    pub id: u64,
    pub owner: Pubkey,
    pub user: Pubkey,
    pub job_title: String,
    pub job_description: String,
    pub amount: u64,
    pub status: STATUS,
}

impl JobDetails {
    pub const MAX_SIZE: usize = 2 + 32 + 1 + (4 + 20) + (4 + 25) + 2 + 1 + 1;
}

#[account]
pub struct ServiceDetails {
    pub id: u64,
    pub owner: Pubkey,
    pub user: Pubkey,
    pub service_title: String,
    pub service_description: String,
    pub amount: u64,
    pub status: STATUS,
    pub bump: u8,
}

impl ServiceDetails {
    pub const MAX_SIZE: usize = 2 + 32 + 1 + (4 + 20) + (4 + 25) + 2 + 1 + 1;
}

#[derive(AnchorDeserialize, AnchorSerialize, PartialEq, Eq, Clone)]
pub enum STATUS {
    OPEN,
    INPROGRESS,
    COMPLETED,
    CLOSED,
    CANCELLED,
}
