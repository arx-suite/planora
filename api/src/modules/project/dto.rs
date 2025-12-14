use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::ProjectRow;

#[derive(Deserialize)]
pub struct CreateProject {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Clone, Deserialize)]
pub struct DeleteProject {
    pub project_id: uuid::Uuid,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectInfo {
    pub project_id: Uuid,
    pub organization_id: Uuid,
    pub space_id: Uuid,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl From<ProjectRow> for ProjectInfo {
    fn from(value: ProjectRow) -> Self {
        Self {
            project_id: value.project_id,
            organization_id: value.organization_id,
            space_id: value.space_id,
            name: value.name,
            description: value.description,
        }
    }
}
