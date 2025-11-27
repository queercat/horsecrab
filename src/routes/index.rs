use crate::{
    database::entities::users::Model, services::topic_service::TopicService,
    utilities::page::TemplateEngine,
};
use rocket::{State, response::content::RawHtml};

#[get("/")]
pub async fn index(topic_service: &State<TopicService>, user: Model) -> RawHtml<String> {
    let mut template_engine = TemplateEngine::default();
    template_engine.set("user", user);

    let posts = topic_service.get_topics().await;
    template_engine.set("posts", posts);

    RawHtml(template_engine.render("index").unwrap())
}
