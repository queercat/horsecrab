use std::{collections::HashMap};
use rocket::response::content::RawHtml;
use crate::utilities::page::Render;

#[get("/login")]
pub async fn login() -> RawHtml<String> {
    let mut environment = HashMap::<String, String>::new();

    let mut page_template = include_str!("../../views/login.template.html").to_string();
    let page = page_template.render(Some(&environment)).unwrap();

    environment.insert("children".to_string(), page);
    environment.insert("error".to_string(), "could not wibble the thing".to_string());

    let mut layout_template = include_str!("../../views/layout.template.html").to_string();
    let mut layout = layout_template.render(Some(&environment)).unwrap();


    RawHtml(layout.render(Some(&environment)).unwrap())
}
