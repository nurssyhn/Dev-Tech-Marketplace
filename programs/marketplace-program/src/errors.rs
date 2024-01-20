use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Job Status Invalid!")]
    InvalidStatus,
    #[msg("Job is not accepting applications anymore!")]
    JobStatusNotOpen,
}
impl From<ErrorCode> for ProgramError {
    fn from(e: ErrorCode) -> Self {
        ProgramError::Custom(e as u32)
    }
}
