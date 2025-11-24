use std::{collections::HashMap, sync::Arc};

use lol_html::{HtmlRewriter, Settings, element, html_content::Element};
use mlua::{Lua, LuaSerdeExt};
use erased_serde::{Serialize};
use rocket::futures::lock::Mutex;

pub trait Render {
    async fn render(&mut self, environment: &Vec<(String, Mutex<Box<dyn Serialize>>)>) -> Result<String, String>;
}

impl Render for String {
    async fn render(&mut self, environment: &Vec<(String, Mutex<Box<dyn Serialize>>)>) -> Result<String, String> {
        let lua = Lua::new();
        let mut env = vec![];

        for (k, v) in environment {
            let value = v.lock().await;
            let value = value.as_ref();
            let value = lua.to_value(&value).unwrap();
            env.push((k.to_owned(), value));
        };

        render(self, env)
    }
}

pub fn render(
    template: &String,
    environment: Vec<(String, mlua::Value)>
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
                    let env = environment.clone();
                    let e = expression.clone();

                    handlers.push(Box::new(move |end| {
                        let end_location = end.source_location().bytes().start;
                        let html = source[start_location..end_location].to_string();

                        let lua = Lua::new();

                        for (k, v) in env {
                            lua.globals()
                                .set(k, v)
                                .expect("Unable to assign globals.")
                        }

                        lua.globals().set("data", html).unwrap();
                        lua.load("function maybe(v, o) return v or o end").exec().expect("Invalid Lua expression.");
                        lua.load("function format(...) data = string.format(data, ...) end").exec().expect("Invalid Lua expression.");
                        lua.load("function each(k) local template = data; data = ''; for _, post in ipairs(k) do data = data .. template:gsub('%$([a-zA-Z_]+)', post) end end").exec().expect("Invalud Lua expression.");
                        lua.load(&e).exec().expect(format!("Invalid Lua expression. {}", e).as_str());

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
