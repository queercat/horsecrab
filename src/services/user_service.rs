use std::sync::Arc;

use rocket::{
    futures::lock::{Mutex, MutexGuard},
};
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, prelude::Uuid
};

use crate::{database::entities::users::{self, Entity as User}, utilities::password::{hash_password, verify_password}};
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

        let hashed_password = hash_password(password.to_string())?;

        let new_user = ActiveUserModel {
            username: Set(username.to_owned()),
            password: Set(hashed_password),
            ..Default::default()
        };

        let users = new_user.insert(&db).await;

        match users {
            Ok(user) => Ok(user),
            _ => Err("Unable to create user!".to_string()),
        }
    }

    pub async fn login_user(&self, username: &str, password: &str) -> bool {
        let db = self.acquire_db().await.clone();

        let user = match User::find().filter(users::Column::Username.eq(username)).one(&db).await {
            Ok(r) => match r {
                Some(u) => u,
                _ => return false
            },
            _ => return false
        };

        verify_password(password.to_string(), user.password)
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
