use std::sync::Arc;

use anyhow::anyhow;
use rocket::futures::lock::{Mutex, MutexGuard};
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter,
    prelude::Uuid,
};

use crate::database::entities::users::ActiveModel as ActiveUserModel;
use crate::database::entities::users::Model as UserModel;
use crate::utilities::auth::JWT;
use crate::utilities::auth::create_jwt;
use crate::{
    database::entities::users::{self, Entity as User},
    utilities::password::{hash_password, verify_password},
};

use crate::services::service_trait::RequiresDatabase;

pub struct UserService {
    db: Arc<Mutex<DatabaseConnection>>,
}

impl RequiresDatabase for UserService {
    async fn acquire_db(&self) -> MutexGuard<'_, DatabaseConnection> {
        self.db.lock().await
    }
}

impl UserService {
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

    pub async fn login_user(&self, username: &str, password: &str) -> anyhow::Result<String> {
        let db = self.acquire_db().await.clone();

        let user = match User::find()
            .filter(users::Column::Username.eq(username))
            .one(&db)
            .await
            .unwrap()
        {
            Some(u) => u,
            _ => return Err(anyhow!("Invalid username or password.")),
        };

        match verify_password(password.to_string(), user.password) {
            true => Ok(create_jwt(user.id)?),
            false => Err(anyhow!("Invalid username or password.")),
        }
    }

    pub async fn get_users(&self) -> Result<Vec<UserModel>, String> {
        let db = self.acquire_db().await.clone();
        let users = User::find().all(&db).await.unwrap();

        Ok(users)
    }

    pub async fn get_user_by_id(&self, id: i64) -> anyhow::Result<UserModel> {
        let db = self.acquire_db().await.clone();
        let user = User::find()
            .filter(users::Column::Id.eq(id))
            .one(&db)
            .await
            .unwrap();

        user.ok_or(anyhow!("No user with id: {}", id))
    }

    pub async fn get_user_from_jwt(&self, jwt: &JWT) -> anyhow::Result<UserModel> {
        let user = self.get_user_by_id(jwt.claims.subject_id).await;

        Ok(user?)
    }
}
