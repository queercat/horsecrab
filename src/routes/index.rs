use erased_serde::Serialize;
use rocket::{State, futures::lock::Mutex, response::content::RawHtml};
use crate::{services::post_service::PostService, utilities::page::Render};

#[get("/index")]
pub async fn index(post_service: &State<PostService>) -> RawHtml<String> {
    let mut environment = Vec::<(String, Mutex<Box<dyn Serialize>>)>::new();

    let mut page_template = include_str!("../../views/index.template.html").to_string();
    let page = page_template.render(&environment).await.unwrap();

    let posts = post_service.get_posts().await;

    environment.push(("children".to_string(), Mutex::new(Box::new(page))));
    environment.push(("posts".to_string(), Mutex::new(Box::new(posts))));

    let mut layout_template = include_str!("../../views/layout.template.html").to_string();
    let mut layout = layout_template.render(&environment).await.unwrap();

    RawHtml(layout.render(&environment).await.unwrap())
}
