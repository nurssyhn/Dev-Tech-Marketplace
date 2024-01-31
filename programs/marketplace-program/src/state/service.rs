use anchor_lang::prelude::*;

use crate::STATUS;
// Struct for service details
#[account]
pub struct Service {
    pub id: Pubkey,
    pub owner: Pubkey,
    pub user: Pubkey,
    pub service_title: String,
    pub service_description: String,
    pub tags: String,
    pub timestamp: i64,
    pub amount: u64,
    pub service_status: STATUS,
    pub payment_status: STATUS,
}

impl Service {
    pub const MAX_SIZE: usize = 3 * 32 + (4 + 50) + (4 + 500) + (4 + 100) + 8 + 8 + 1;

    pub fn init_service(
        &mut self,
        id: Pubkey,
        owner: Pubkey,
        service_title: String,
        service_description: String,
        tags: String,
        amount: u64,
    ) -> Result<()> {
        let clock = Clock::get()?;
        self.id = id;
        self.owner = owner;
        self.service_title = service_title;
        self.service_description = service_description;
        self.tags = tags;
        self.timestamp = clock.unix_timestamp;
        self.amount = amount;
        self.service_status = STATUS::OPEN;
        self.payment_status = STATUS::OPEN;
        Ok(())
    }

    pub fn buy_service(
        &mut self,
        id: Pubkey,
        owner: Pubkey,
        user: Pubkey,
        service_title: String,
        service_description: String,
        tags: String,
        amount: u64,
    ) -> Result<()> {
        let clock = Clock::get()?;
        self.id = id;
        self.owner = owner;
        self.user = user;
        self.service_title = service_title;
        self.service_description = service_description;
        self.tags = tags;
        self.timestamp = clock.unix_timestamp;
        self.amount = amount;
        self.service_status = STATUS::INPROGRESS;
        self.payment_status = STATUS::INPROGRESS;
        Ok(())
    }
}
