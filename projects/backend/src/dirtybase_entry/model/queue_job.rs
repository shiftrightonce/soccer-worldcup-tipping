use dirtybase_app::{
    db::{
        field_values::FieldValue,
        types::{ArcUuid7, CreatedAtField, StringField, UpdatedAtField},
    },
    db_macro::DirtyTable,
};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum JobStatus {
    #[default]
    Pending,
    InProgress,
    Completed,
    Failed,
}

#[derive(Debug, Clone, Default, DirtyTable)]
#[dirty(timestamps, id_not_auto)]
pub struct QueueJob {
    pub id: Option<ArcUuid7>,
    pub job_type: StringField,
    pub payload: Option<StringField>,
    pub status: JobStatus,
    pub log: Option<StringField>,
    pub created_at: CreatedAtField,
    pub updated_at: UpdatedAtField,
}

impl From<JobStatus> for FieldValue {
    fn from(status: JobStatus) -> Self {
        match status {
            JobStatus::Pending => FieldValue::String("pending".to_string()),
            JobStatus::InProgress => FieldValue::String("in_progress".to_string()),
            JobStatus::Completed => FieldValue::String("completed".to_string()),
            JobStatus::Failed => FieldValue::String("failed".to_string()),
        }
    }
}

impl From<FieldValue> for JobStatus {
    fn from(value: FieldValue) -> Self {
        match value {
            FieldValue::String(s) => match s.as_str() {
                "pending" => JobStatus::Pending,
                "in_progress" => JobStatus::InProgress,
                "completed" => JobStatus::Completed,
                "failed" => JobStatus::Failed,
                _ => panic!("Invalid job status string: {}", s),
            },
            _ => panic!("Expected a string for job status, got: {:?}", value),
        }
    }
}
