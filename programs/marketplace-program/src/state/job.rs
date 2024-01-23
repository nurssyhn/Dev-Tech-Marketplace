use anchor_lang::prelude::*;
// Struct for job details
#[account]
pub struct Job {
    pub id: Pubkey,
    pub owner: Pubkey,
    pub user: Pubkey,
    pub job_title: String,
    pub job_description: String,
    pub tags: String,
    pub timestamp: i64,
    pub amount: u64,
    pub status: STATUS,
    pub bidders: Vec<Pubkey>,
}

impl Job {
    pub const MAX_SIZE: usize =
        32 + 32 + 32 + (4 + 20) + (4 + 25) + (4 + 25) + 8 + 8 + 1 + (10 * 32) + 1;

    pub fn init_job(
        &mut self,
        id: Pubkey,
        owner: Pubkey,
        job_title: String,
        job_description: String,
        tags: String,
        amount: u64,
    ) -> Result<()> {
        let clock = Clock::get()?;
        self.id = id;
        self.owner = owner;
        self.job_title = job_title;
        self.job_description = job_description;
        self.tags = tags;
        self.timestamp = clock.unix_timestamp;
        self.amount = amount;
        self.status = STATUS::OPEN;
        Ok(())
    }
}

#[derive(AnchorDeserialize, AnchorSerialize, PartialEq, Eq, Clone)]
pub enum STATUS {
    OPEN,
    INPROGRESS,
    COMPLETED,
    PAID,
    CANCELLED,
}
