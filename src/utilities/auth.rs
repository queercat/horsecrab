use anyhow::anyhow;
use chrono::Utc;
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::serde::{Deserialize, Serialize};
use rocket::{Request, request};
use std::env;

use crate::database::entities::users;
use crate::services::user_service::UserService;

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    pub subject_id: i64,
    exp: usize,
}

#[derive(Debug)]
pub struct JWT {
    pub claims: Claims,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for users::Model {
    type Error = anyhow::Error;
    
    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, anyhow::Error> {
        let user_service = request
            .rocket()
            .state::<UserService>()
            .expect("Unable to acquire user service");

        let cookie_jar = request.cookies();

        let encoded_jwt = match cookie_jar.get("authorization") {
            Some(h) => h.value(),
            _ => return Outcome::Error((Status::Unauthorized, anyhow!("Unauthorized."))),
        };

        let jwt = match decode_jwt(encoded_jwt) {
            Ok(jwt) => jwt,
            _ => { 
                cookie_jar.remove("authorization");
                return Outcome::Error((Status::Unauthorized, anyhow!("Unauthorized.")))
            },
        };

        let user = match user_service.get_user_from_jwt(&jwt).await {
            Ok(u) => u,
            _ => return Outcome::Error((Status::Unauthorized, anyhow!("Unauthorized."))),
        };

        rocket::outcome::Outcome::Success(user)
    }
}

pub fn create_jwt(id: i64) -> anyhow::Result<String> {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set.");

    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::seconds(60))
        .expect("Invalid timestamp")
        .timestamp();

    let claims = Claims {
        subject_id: id,
        exp: expiration as usize,
    };

    let header = Header::new(Algorithm::HS512);

    Ok(encode(
        &header,
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )?)
}

pub fn decode_jwt(encoded_jwt: &str) -> anyhow::Result<JWT> {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set.");
    let token = encoded_jwt.trim();

    let token = decode::<Claims>(
        &token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::new(Algorithm::HS512),
    );

    Ok(JWT {
        claims: token?.claims,
    })
}
