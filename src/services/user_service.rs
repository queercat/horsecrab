use std::sync::Arc;

use rocket::{
    futures::lock::{Mutex, MutexGuard},
};
use sea_orm::{
    ActiveModelTrait, ActiveValue::{self, Set}, ConnectionTrait, DatabaseConnection, EntityTrait, sea_query::{Cond, Query}
};

use crate::database::entities::users::Entity as User;
use crate::database::entities::users::Model as UserModel;
use crate::database::entities::users::ActiveModel as ActiveUserModel;

use crate::{services::service_trait::RequiresDatabase};

pub struct UserService {
    db: Arc<Mutex<DatabaseConnection>>,
}

impl RequiresDatabase for UserService {
    async fn acquire_db(&self) -> MutexGuard<'_, DatabaseConnection> {
        self.db.lock().await
    }
}

impl UserService  {
    pub fn new(db: Arc<Mutex<DatabaseConnection>>) -> Self {
        Self { db }
    }

    pub async fn create_user(&self, username: &str, password: &str) -> Result<UserModel, String> {
        let db = self.acquire_db().await.clone();

        let new_user = ActiveUserModel {
            username: Set(username.to_owned()),
            password: Set(password.as_bytes().to_vec().to_owned()),
            ..Default::default()
        };

        let users = new_user.insert(&db).await;

        match users {
            Ok(user) => Ok(user),
            _ => Err("Unable to create user!".to_string()),
        }
    }

    pub async fn get_users(&self) -> Result<Vec<UserModel>, String> {
        let db = self.acquire_db().await.clone();
        let users = User::find().all(&db).await;

        match users {
            Ok(users) => Ok(users),
            _ => Err("Unable to create user!".to_string()),
        }
    }
}
