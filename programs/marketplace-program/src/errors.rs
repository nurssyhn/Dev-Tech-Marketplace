use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Job Status Invalid!")]
    InvalidStatus,
    #[msg("Invalid User!")]
    UserNotAuthorized,
    #[msg("Job is not accepting applications anymore!")]
    JobStatusNotOpen,
    #[msg("Service is available anymore!")]
    ServiceStatusNotOpen,
}
impl From<ErrorCode> for ProgramError {
    fn from(e: ErrorCode) -> Self {
        ProgramError::Custom(e as u32)
    }
}
