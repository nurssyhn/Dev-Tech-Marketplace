use::anchor_lang::prelude::*;

#[account]
pub struct DeveloperMarketplace {
    pub admin: Pubkey,       
    pub fee: u16,            
    pub name: String,        
}

impl Space for DeveloperMarketplace {
    const INIT_SPACE: usize = 8 + 32 + 2 + 4; 
}

#[account]
pub struct DeveloperListing {
    pub owner: Pubkey,      // Dev's wallet
    pub skills: Vec<String>, 
    pub hourly_rate: u64,    
    pub experience: u64,     
    // Add more fields here
}

impl Space for DeveloperListing {
    const INIT_SPACE: usize = 8 + 32 + (64 * SKILLS_MAX) + 8 + 8; 
}

#[account]
pub struct JobListing {
    // Details abot the job listing
}


//servicelisting
//user accounts -> enum: employer, developer (add abstract) add parameters as state 

/*pub enum EmployeeType {
    Employer,
    Employee
    }
    #[account]
    pub struct User {
    type: EmployeeType
*/
