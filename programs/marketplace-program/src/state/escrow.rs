use anchor_lang::prelude::*;

#[account]
pub struct Escrow {
    pub employer: Pubkey, //maker
    pub employee: Pubkey, //taker
    pub product: Pubkey,  // job or service id
    pub amount: u64,
    pub is_initialized: bool,
}

impl Escrow {
    pub const MAX_SIZE: usize = 32 * 3 + 8 + 1;

    pub fn init_escrow(
        &mut self,
        employer: Pubkey,
        employee: Pubkey,
        product: Pubkey,
        amount: u64,
        is_initialized: bool,
    ) -> Result<()> {
        self.employer = employer;
        self.employee = employee;
        self.product = product;
        self.amount = amount;
        self.is_initialized = is_initialized;
        Ok(())
    }
}
