use rshtml::{RsHtml};

#[derive(RsHtml)]
pub struct HomePage {
    pub title: String,
}

#[derive(RsHtml)]
pub struct IndexPage {}

#[derive(RsHtml)]
pub struct RegistrationPage {}

#[derive(RsHtml)]
pub struct LoginPage {}