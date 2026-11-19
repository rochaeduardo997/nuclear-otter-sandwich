use crate::domain::project::entity::project::SProject;

pub trait IProjectRepository: Send + Sync {
    fn create(
        &self,
        project: SProject,
        session_id: uuid::Uuid,
        user_id: Option<uuid::Uuid>,
    ) -> Result<SProject, anyhow::Error>;
    fn update(
        &self,
        project: SProject,
        session_id: uuid::Uuid,
        user_id: Option<uuid::Uuid>,
    ) -> Result<SProject, anyhow::Error>;
}
