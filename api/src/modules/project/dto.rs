use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateProject {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Clone, Deserialize)]
pub struct DeleteProject {
    pub project_id: uuid::Uuid,
}
