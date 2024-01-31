use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid User!")]
    UserNotAuthorized,
    #[msg("Job Status Invalid!")]
    InvalidJobStatus,
    #[msg("ServiceStatus Invalid!")]
    InvalidServiceStatus,
    #[msg("Job is not accepting applications anymore!")]
    JobStatusNotOpen,
    #[msg("Service is not available currently!")]
    ServiceStatusNotOpen,
    #[msg("This job is not available anymore!")]
    JobStatusCompleted,
}
impl From<ErrorCode> for ProgramError {
    fn from(e: ErrorCode) -> Self {
        ProgramError::Custom(e as u32)
    }
}
