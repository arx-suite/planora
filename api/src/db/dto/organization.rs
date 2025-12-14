use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::db::models::OrganizationRow;

#[derive(Debug, Clone, Deserialize)]
pub struct CreateOrg {
    pub name: String,
    pub subdomain: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct OrgProfile {
    organization_id: Uuid,
    owner_id: Uuid,
    name: String,
    subdomain: String,
    plan: String,
}

impl From<OrganizationRow> for OrgProfile {
    fn from(value: OrganizationRow) -> Self {
        Self {
            organization_id: value.organization_id,
            owner_id: value.owner_id,
            name: value.name,
            subdomain: value.subdomain,
            plan: value.plan,
        }
    }
}
