use rshtml::{RsHtml};

#[derive(RsHtml)]
pub struct HomePage {
    pub title: String,
    pub error: String
}

#[derive(RsHtml)]
pub struct IndexPage {
    pub error: String
}

#[derive(RsHtml)]
pub struct RegistrationPage {
    pub error: String
}

#[derive(RsHtml)]
pub struct LoginPage {
    pub error: String
}