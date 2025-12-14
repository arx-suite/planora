use sea_query::{Asterisk, Expr, PostgresQueryBuilder, Query};
use sqlx::{self, PgPool};
use tracing::{debug, info, trace};
use uuid::Uuid;

use super::{DeleteSpace, NewSpace, SpaceRow, Spaces, UpdateSpace};
use crate::db::{DBResult, helpers::with_org};

pub struct SpaceRepo<'a> {
    pub pool: &'a PgPool,
}

impl<'a> SpaceRepo<'a> {
    pub fn new(pool: &'a PgPool) -> Self {
        Self { pool }
    }

    /* create */
    pub async fn create_space(&self, new_space: NewSpace, org_id: Uuid) -> DBResult<SpaceRow> {
        trace!(space_name = ?new_space.space_name, organization_id = ?org_id, "starting space creation");

        let query = Query::insert()
            .into_table(Spaces::Table)
            .columns([Spaces::SpaceName, Spaces::Description])
            .values([new_space.space_name.into(), new_space.description.into()])?
            .returning_all()
            .to_string(PostgresQueryBuilder);

        debug!(query = %query, "generated insert query");

        let inserted_space = with_org(self.pool, &org_id, |mut tx| async move {
            let inserted = sqlx::query_as::<_, SpaceRow>(&query)
                .fetch_one(&mut *tx)
                .await?;

            Ok((inserted, tx))
        })
        .await?;

        info!(
            organization_id = ?inserted_space.organization_id,
            space_name = ?inserted_space.space_name,
            "successfully created new space"
        );

        Ok(inserted_space)
    }

    /* read */
    pub async fn find_by_space_id(
        &self,
        space_id: Uuid,
        org_id: Uuid,
    ) -> DBResult<Option<SpaceRow>> {
        trace!(space_id = ?space_id, organization_id = ?org_id, "find space by id");

        let query = Query::select()
            .column(Asterisk)
            .from(Spaces::Table)
            .and_where(Expr::col(Spaces::SpaceId).eq(space_id))
            .to_string(PostgresQueryBuilder);

        debug!(query = %query, "generated select query");

        let space = with_org(self.pool, &org_id, |mut tx| async move {
            let spaces = sqlx::query_as::<_, SpaceRow>(&query)
                .fetch_optional(&mut *tx)
                .await?;

            Ok((spaces, tx))
        })
        .await?;

        Ok(space)
    }

    pub async fn find_by_org_id(&self, org_id: Uuid) -> DBResult<Vec<SpaceRow>> {
        trace!(organization_id = ?org_id, "find spaces by org_id");

        let query = Query::select()
            .column(Asterisk)
            .from(Spaces::Table)
            .to_string(PostgresQueryBuilder);

        debug!(query = %query, "generated select query");

        let spaces = with_org(self.pool, &org_id, |mut tx| async move {
            let spaces = sqlx::query_as::<_, SpaceRow>(&query)
                .fetch_all(&mut *tx)
                .await?;

            Ok((spaces, tx))
        })
        .await?;

        Ok(spaces)
    }

    /* update */
    pub async fn update_space(
        &self,
        update_space: UpdateSpace,
        org_id: Uuid,
    ) -> DBResult<SpaceRow> {
        trace!(space_name = ?update_space.space_name, description = ?update_space.description, "update spaces");

        let mut builder = Query::update();
        builder.table(Spaces::Table);

        update_space
            .space_name
            .map(|name| builder.value(Spaces::SpaceName, name));
        update_space
            .description
            .map(|description| builder.value(Spaces::Description, description));

        builder.returning_all();
        builder.and_where(Expr::col(Spaces::SpaceId).eq(update_space.space_id));

        let query = builder.to_string(PostgresQueryBuilder);
        debug!(query = %query, "generated update query");

        let space = with_org(self.pool, &org_id, |mut tx| async move {
            let space = sqlx::query_as::<_, SpaceRow>(&query)
                .fetch_one(&mut *tx)
                .await?;

            Ok((space, tx))
        })
        .await?;

        Ok(space)
    }

    /* delete */
    pub async fn delete_by_space_id(
        &self,
        delete_space: DeleteSpace,
        org_id: Uuid,
    ) -> DBResult<u64> {
        trace!(organization_id = ?delete_space.space_id, "delete space by id");

        let query = Query::update()
            .table(Spaces::Table)
            .and_where(Expr::col(Spaces::SpaceId).eq(delete_space.space_id))
            .to_string(PostgresQueryBuilder);

        debug!(query = %query, "generated query");

        let result = with_org(self.pool, &org_id, |mut tx| async move {
            let result = sqlx::query(&query).execute(&mut *tx).await?;
            Ok((result, tx))
        })
        .await?;

        Ok(result.rows_affected())
    }
}
