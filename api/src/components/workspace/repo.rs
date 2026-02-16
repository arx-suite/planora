#![allow(unused_variables)]

use sqlx::PgExecutor;
use uuid::Uuid;

use super::model::{OrganizationFeatureRow, OrganizationResourceRow};
use crate::services::db::DBResult;

#[async_trait::async_trait]
pub trait FeaturesRepo {
    // resources
    async fn resources(&self, org_id: Uuid) -> DBResult<OrganizationResourceRow>;

    // features
    async fn features(&self, org_id: Uuid) -> DBResult<Vec<OrganizationFeatureRow>>;
    async fn feature_enable(
        &self,
        org_id: Uuid,
        user_id: Uuid,
        feature_key: String,
    ) -> DBResult<()>;
    async fn feature_disable(
        &self,
        org_id: Uuid,
        user_id: Uuid,
        feature_key: String,
    ) -> DBResult<()>;
    async fn upgrade_plan(&self, org_id: Uuid) -> DBResult<()>;
}

#[async_trait::async_trait]
impl<T> FeaturesRepo for T
where
    for<'e> &'e T: PgExecutor<'e>,
{
    async fn resources(&self, org_id: Uuid) -> DBResult<OrganizationResourceRow> {
        todo!()
    }

    async fn features(&self, org_id: Uuid) -> DBResult<Vec<OrganizationFeatureRow>> {
        todo!()
    }

    async fn feature_enable(
        &self,
        org_id: Uuid,
        user_id: Uuid,
        feature_key: String,
    ) -> DBResult<()> {
        todo!()
    }

    async fn feature_disable(
        &self,
        org_id: Uuid,
        user_id: Uuid,
        feature_key: String,
    ) -> DBResult<()> {
        todo!()
    }

    async fn upgrade_plan(&self, org_id: Uuid) -> DBResult<()> {
        todo!()
    }
}
