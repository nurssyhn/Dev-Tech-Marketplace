use anchor_lang::error_code;

#[error_code(1000)]
pub enum MarketplaceError{
    #[msg("Not the right Token Standard")]
    InvalidTokenStandard,

    #[msg("Not the right Collection")]
    InvalidCollection,
}