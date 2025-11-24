use std::sync::Arc;

use rocket::futures::lock::{Mutex, MutexGuard};
use sea_orm::{DatabaseConnection, EntityTrait};

use crate::{
    database::entities::topics::{ActiveModel, Entity, Model},
    services::service_trait::RequiresDatabase,
};

pub struct TopicService {
    db: Arc<Mutex<DatabaseConnection>>,
}

impl RequiresDatabase for TopicService {
    async fn acquire_db(&self) -> MutexGuard<'_, DatabaseConnection> {
        self.db.lock().await
    }
}

impl TopicService {
    pub fn new(db: Arc<Mutex<DatabaseConnection>>) -> Self {
        Self { db }
    }

    pub async fn get_topics(&self) -> Vec<Model> {
        let db = self.acquire_db().await.clone();
        let posts = Entity::find().all(&db).await;

        posts.unwrap()
    }
}
