use crate::domain::project::value_object::execution::SExecution;
use crate::domain::project::value_object::folder::SFolder;
use crate::domain::project::value_object::language::ELanguage;

#[derive(Debug, Clone)]
pub struct SProject {
    pub id: uuid::Uuid,
    pub session_id: uuid::Uuid,
    pub name: String,
    pub language: ELanguage,
    pub folders: Vec<SFolder>,
    pub executions_history: Option<Vec<SExecution>>,
}
