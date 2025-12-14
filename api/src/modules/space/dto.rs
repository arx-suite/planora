use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::SpaceRow;

#[derive(Debug, Deserialize)]
pub struct NewSpace {
    pub space_name: String,
    pub description: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct DeleteSpace {
    pub space_id: Uuid,
}

#[derive(Debug, Deserialize)]
pub struct UpdateSpace {
    pub space_id: Uuid,
    pub space_name: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SpaceInfo {
    pub space_id: Uuid,
    pub organization_id: Uuid,
    pub space_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl From<SpaceRow> for SpaceInfo {
    fn from(value: SpaceRow) -> Self {
        Self {
            space_id: value.space_id,
            organization_id: value.organization_id,
            space_name: value.space_name,
            description: value.description,
        }
    }
}
