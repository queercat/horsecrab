use rocket::Route;

pub fn get_routes() -> Vec<Route> {
    routes![home::home, index::index, layout::layout, login::login, registration::registration]
}

pub mod home;
pub mod index;
pub mod layout;
pub mod login;
pub mod registration;
