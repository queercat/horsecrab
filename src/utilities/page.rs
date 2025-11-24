use std::collections::HashMap;

use lol_html::{HtmlRewriter, Settings, element, html_content::Element};
use mlua::Lua;

pub trait Render {
    fn render(&mut self, environment: Option<&HashMap<String, String>>) -> Result<String, String>;
}

impl Render for String {
    fn render(&mut self, environment: Option<&HashMap<String, String>>) -> Result<String, String> {
        render(self.to_owned(), environment)
    }
}

pub fn render(
    template: String,
    environment: Option<&HashMap<String, String>>,
) -> Result<String, String> {
    let mut buffer = vec![];
    let mut rewriter = HtmlRewriter::new(
        Settings {
            element_content_handlers: vec![element!("lua", |el: &mut Element| {
                let start_location = el.source_location().bytes().end;
                let expression = el.get_attribute("code").unwrap_or("".to_string());
                el.remove();
                if let Some(handlers) = el.end_tag_handlers() {
                    let source = template.clone();
                    let env = match environment {
                        Some(v) => Some(v.clone()),
                        None => None,
                    };
                    let e = expression.clone();

                    handlers.push(Box::new(move |end| {
                        let end_location = end.source_location().bytes().start;
                        let html = source[start_location..end_location].to_string();

                        let lua = Lua::new();

                        if let Some(e) = env {
                            for (key, value) in e.into_iter() {
                                lua.globals()
                                    .set(key.to_string(), value.to_string())
                                    .expect("Unable to assign globals.")
                            }
                        }

                        lua.globals().set("data", html).unwrap();
                        lua.load(e).exec().expect("Invalid Lua expression.");

                        let data: String = lua.globals().get("data").unwrap();

                        end.before(&data, lol_html::html_content::ContentType::Html);

                        Ok(())
                    }));
                }
                Ok(())
            })],
            ..Settings::new()
        },
        |c: &[u8]| buffer.extend_from_slice(c),
    );

    rewriter.write(template.as_bytes()).unwrap();

    Ok(String::from_utf8(buffer).unwrap())
}
