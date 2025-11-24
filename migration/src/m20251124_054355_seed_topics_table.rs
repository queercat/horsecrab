use sea_orm_migration::{prelude::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let statement = Query::insert()
            .into_table("topics")
            .columns(["title", "description", "id"])
            .values_panic(["memes".into(), "this is where the memes are".into(), 0.into()])
            .to_owned();

        manager.exec_stmt(statement).await?;


        let statement = Query::insert()
            .into_table("topics")
            .columns(["title", "description", "id"])
            .values_panic(["sad".into(), "this is where the sad are".into(), 1.into()])
            .to_owned();

        manager.exec_stmt(statement).await?;

        
        let statement = Query::insert()
            .into_table("topics")
            .columns(["title", "description", "id"])
            .values_panic(["ponies".into(), "this is where the pony are".into(), 2.into()])
            .to_owned();

        manager.exec_stmt(statement).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let statement = Query::delete().from_table("topics").cond_where(Cond::any().add(Expr::col("id").eq(0))).to_owned();
        let statement = Query::delete().from_table("topics").cond_where(Cond::any().add(Expr::col("id").eq(1))).to_owned();
        let statement = Query::delete().from_table("topics").cond_where(Cond::any().add(Expr::col("id").eq(2))).to_owned();

        manager.exec_stmt(statement).await?;

        Ok(())
    }
}
