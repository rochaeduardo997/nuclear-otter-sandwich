use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub enum EExecutionStatus {
    Success,
    Failure,
}

#[derive(Debug, Clone)]
pub struct SExecution {
    pub at: DateTime<Utc>,
    pub started_at: DateTime<Utc>,
    pub finished_at: Option<DateTime<Utc>>,
    pub status: EExecutionStatus,
    pub error_code: Option<String>,
    pub error_message: Option<String>,
    pub execution_code: String,
}
