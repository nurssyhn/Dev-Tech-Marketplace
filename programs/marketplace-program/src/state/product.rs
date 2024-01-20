use crate::errors::ErrorCode;
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
    pub status: STATUS,
}

impl Service {
    pub const MAX_SIZE: usize = 2 + 32 + 32 + 1 + (4 + 20) + (4 + 25) + (4 + 25) + 8 + 8 + 1 + 1;

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
        self.status = STATUS::OPEN;
        Ok(())
    }
}

#[derive(AnchorDeserialize, AnchorSerialize, PartialEq, Eq, Clone)]
pub enum STATUS {
    OPEN,
    INPROGRESS,
    COMPLETED,
    CLOSED,
    CANCELLED,
}

// impl STATUS {
//     pub fn to_code(&self) -> u8 {
//         match self {
//             STATUS::OPEN => 0,
//             STATUS::INPROGRESS => 1,
//             STATUS::COMPLETED => 2,
//             STATUS::CLOSED => 3,
//             STATUS::CANCELLED => 4,
//         }
//     }

//     pub fn from(val: u8) -> std::result::Result<STATUS, ProgramError> {
//         match val {
//             0 => Ok(STATUS::OPEN),
//             1 => Ok(STATUS::INPROGRESS),
//             2 => Ok(STATUS::COMPLETED),
//             3 => Ok(STATUS::CLOSED),
//             4 => Ok(STATUS::CANCELLED),
//             _ => Err(ErrorCode::InvalidStatus.into()),
//         }
//     }
// }
