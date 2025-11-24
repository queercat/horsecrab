use crate::{database::entities::users::Model, services::topic_service::TopicService, utilities::page::Render};
use erased_serde::Serialize;
use rocket::{State, response::content::RawHtml};
use std::sync::Mutex;


#[get("/index")]
pub async fn index(topic_service: &State<TopicService>, user: Model) -> RawHtml<String> {
    let mut environment = Vec::<(String, Mutex<Box<dyn Serialize + Send>>)>::new();

    let posts = topic_service.get_topics().await;
    environment.push(("posts".to_string(), Mutex::new(Box::new(posts))));

    let mut page_template = include_str!("../../views/index.template.html").to_string();
    let page = page_template.render(&environment).unwrap();

    environment.push(("children".to_string(), Mutex::new(Box::new(page))));

    let mut layout_template = include_str!("../../views/layout.template.html").to_string();
    let mut layout = layout_template.render(&environment).unwrap();

    RawHtml(layout.render(&environment).unwrap())
}

