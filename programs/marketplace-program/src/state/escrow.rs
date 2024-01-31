use anchor_lang::prelude::*;

#[account]
pub struct Escrow {
    pub seed: u64,
    pub maker: Pubkey,   //maker
    pub taker: Pubkey,   //taker
    pub product: Pubkey, // job or service id
    pub amount: u64,
    pub is_initialized: bool,
}

impl Escrow {
    pub const MAX_SIZE: usize = 8 + 32 * 3 + 8 + 1;

    pub fn init_escrow(
        &mut self,
        seed: u64,
        maker: Pubkey,
        taker: Pubkey,
        product: Pubkey,
        amount: u64,

        is_initialized: bool,
    ) -> Result<()> {
        self.seed = seed;
        self.maker = maker;
        self.taker = taker;
        self.product = product;
        self.amount = amount;
        self.is_initialized = is_initialized;
        Ok(())
    }
}
