use rocket::futures::lock::MutexGuard;
use sea_orm::DatabaseConnection;

pub trait RequiresDatabase {
    async fn acquire_db(&self) -> MutexGuard<'_, DatabaseConnection>;
}