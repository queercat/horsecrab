#[macro_use]
extern crate rocket;

mod database;
mod models;
mod services;
mod utilities;

use std::sync::Arc;

use rocket::State;
use rocket::form::Form;
use rocket::futures::lock::Mutex;
use rocket::{fs::FileServer, response::content::RawHtml};
use rshtml::traits::RsHtml;

use crate::database::setup::set_up_db;
use crate::models::pages::{HomePage, LoginPage, RegistrationPage};
use crate::services::user_service::UserService;
use crate::utilities::page::render;

#[get("/")]
async fn index(user_service: &State<UserService>) -> RawHtml<String> {
    let result = user_service.get_users().await;

    let mut page = HomePage {
        error: "".to_string(),
        title: if result.is_ok() {
            "ok".to_string()
        } else {
            "not ok".to_string()
        },
    };

    RawHtml(page.render().unwrap())
}

#[get("/register")]
async fn register() -> RawHtml<String> {
    let mut page = RegistrationPage {
        error: "".to_string()
    };

    RawHtml(page.render().unwrap())
}

#[get("/login")]
async fn login() -> RawHtml<String> {
    let mut page = LoginPage {
        error: "".to_string()
    };

    RawHtml(page.render().unwrap())
}

#[post("/register", data = "<registration_request>")]
async fn handle_register(
    user_service: &State<UserService>,
    registration_request: Form<models::requests::RegistrationRequest<'_>>,
) -> RawHtml<String> {
    let result = user_service
        .create_user(registration_request.username, registration_request.password)
        .await;

    let error = match result {
        Ok(_) => "".to_string(),
        Err(s) => s
    };

    let mut page = RegistrationPage {
        error
    };
    RawHtml(page.render().unwrap())
}

#[post("/login", data = "<login_request>")]
async fn handle_login(
    user_service: &State<UserService>,
    login_request: Form<models::requests::LoginRequest<'_>>,
) -> RawHtml<String> {
    let result = user_service.login_user(login_request.username, login_request.password).await;

    let mut page = Box::new(LoginPage {
        error: if result { "true".to_string() } else { "false".to_string() }
    }) as Box<dyn RsHtml + 'static>;

    RawHtml(render(&mut page).unwrap())
}

#[launch]
async fn rocket() -> _ {
    dotenv::dotenv().ok();

    let db = match set_up_db().await {
        Ok(db) => Arc::new(Mutex::new(db)),
        _ => panic!("Unable to initialize database"),
    };

    let user_service = UserService::new(Arc::clone(&db));

    rocket::build()
        .manage(user_service)
        .mount("/", routes![index, register, handle_register, login, handle_login])
        .mount("/", FileServer::from("./public"))
}
