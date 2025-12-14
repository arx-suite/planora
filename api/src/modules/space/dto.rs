use serde::Deserialize;
use uuid::Uuid;

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
