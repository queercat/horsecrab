#[macro_use]
extern crate rocket;

mod database;
mod models;
mod routes;
mod services;
mod utilities;

use std::sync::Arc;

use rocket::State;
use rocket::form::Form;
use rocket::futures::lock::Mutex;
use rocket::http::{Cookie, CookieJar};
use rocket::{fs::FileServer, response::content::RawHtml};

use crate::database::setup::set_up_db;
use crate::routes::get_routes;
use crate::services::topic_service::TopicService;
use crate::services::user_service::UserService;

#[post("/register", data = "<registration_request>")]
async fn handle_register(
    user_service: &State<UserService>,
    registration_request: Form<models::requests::RegistrationRequest<'_>>,
) -> RawHtml<String> {
    let result = user_service
        .create_user(registration_request.username, registration_request.password)
        .await
        .unwrap();

    RawHtml("".to_string())
}

#[post("/login", data = "<login_request>")]
async fn handle_login(
    user_service: &State<UserService>,
    login_request: Form<models::requests::LoginRequest<'_>>,
    jar: &CookieJar<'_>,
) -> RawHtml<String> {
    let jwt = user_service
        .login_user(login_request.username, login_request.password)
        .await
        .unwrap();

    let authorization_cookie = Cookie::new("authorization", jwt);

    jar.add(authorization_cookie);

    RawHtml("".to_string())
}

#[launch]
async fn rocket() -> _ {
    dotenv::dotenv().ok();

    let db = match set_up_db().await {
        Ok(db) => Arc::new(Mutex::new(db)),
        _ => panic!("Unable to initialize database"),
    };

    let user_service = UserService::new(Arc::clone(&db));
    let post_service = TopicService::new(Arc::clone(&db));

    rocket::build()
        .manage(user_service)
        .manage(post_service)
        .mount("/", get_routes())
        .mount("/", routes![handle_login, handle_register])
        .mount("/", FileServer::from("./public"))
}
