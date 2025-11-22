use sea_orm_migration::{prelude::*, schema::*, sea_orm::Statement};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let statement = Query::insert()
            .into_table("users")
            .columns(["id", "username", "password"])
            .values_panic([0.into(), "admin".into(), vec![0].into()])
            .to_owned();

        manager.exec_stmt(statement).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let statement = Query::delete().from_table("users").cond_where(Cond::any().add(Expr::col("id").eq(1))).to_owned();

        manager.exec_stmt(statement).await?;

        Ok(())
    }
}
