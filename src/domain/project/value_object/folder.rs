use super::file::SFile;

#[derive(Debug, Clone)]
pub struct SFolder {
    pub id: uuid::Uuid,
    pub name: String,
    pub folder: Vec<SFolder>,
    pub files: Vec<SFile>,
    pub path: String,
}
