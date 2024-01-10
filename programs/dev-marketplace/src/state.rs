use::anchor_lang::prelude::*;

// Marketplace account structure containing general information about the marketplace.
#[account]
pub struct DevMarketplace {
    pub admin: Pubkey, // Admin's wallet address
    pub fee_percentage: u16, // Transaction fee percentage
    pub name: String,  // Marketplace name
    // Additional settings can be added here.
}

impl Space for DevMarketplace {
    const INIT_SPACE: usize = 8 + 32 + 2 + 64; 
}

// Developer account structure holding profile information and work history.
#[account]
pub struct DevAccount {
    pub owner: Pubkey, // Developer's wallet address
    pub profile_info: String, // Profile information
    // Other attributes like experience, skills, completed projects, etc.
}

impl Space for DevAccount {
    const INIT_SPACE: usize = 8 + 32 + 256; 
}

// Client account structure containing information about job postings and interaction history.
#[account]
pub struct ClientAccount {
    pub owner: Pubkey, // Client's wallet address
    // Other attributes like ongoing projects, interaction history, etc.
}

impl Space for ClientAccount{
    const INIT_SPACE: usize = 8 + 32; 
}

// Listing account structure for developers to list their services and skills.
#[account]
pub struct Listing {
    pub owner: Pubkey, // Developer's wallet address associated with the listing
    pub services: String, // Services offered
    pub availability: bool, // Availability for new projects
    // Additional attributes like rates, client reviews, etc.
}

impl Space for Listing{
    const INIT_SPACE: usize = 8 + 32 + 256 + 1; 
}


// Job posting structure for clients to post job details.
#[derive(Clone)]
pub struct JobPosting {
    pub client: Pubkey, // Client's wallet address who posted the job
    pub details: String, // Details of the job
    // Additional attributes like budget, duration, requirements, etc.
}

// Transaction or contract account structure to manage job agreements and payments.
#[account]
pub struct Transaction {
    pub developer: Pubkey, // Developer's wallet address
    pub client: Pubkey, // Client's wallet address
    pub amount: u64, // Payment amount
    // Additional attributes like payment status, agreement details, etc.
}
