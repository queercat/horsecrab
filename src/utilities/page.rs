use std::{
    fmt::Display,
    fs,
    path::{Path, PathBuf},
    sync::Mutex,
};

use erased_serde::Serialize;
use lol_html::{HtmlRewriter, Settings, element, html_content::Element};
use mlua::{Lua, LuaSerdeExt};

pub struct TemplateEngine {
    base_path: PathBuf,
    layout_path: PathBuf,
    use_layout: bool,
    environment: Vec<(String, Mutex<Box<dyn Serialize + Send>>)>,
}

impl TemplateEngine {
    fn get_path_to_template(&self, template_name: &impl AsRef<Path>) -> PathBuf {
        self.base_path
            .join(template_name.as_ref().with_extension("template.html"))
            .to_path_buf()
    }

    pub fn render(&mut self, template_name: impl AsRef<Path>) -> anyhow::Result<String> {
        let path = self.get_path_to_template(&template_name);
        let mut template = fs::read_to_string(path)?;
        let mut rendered = template.render(&self.environment)?;

        if self.use_layout {
            self.set("children", rendered);
            template = fs::read_to_string(self.get_path_to_template(&self.layout_path))?;
            rendered = template.render(&self.environment)?;
            self.delete("children");
        }

        Ok(rendered)
    }

    pub fn set<K: Display, T: Serialize + Send + 'static>(&mut self, key: K, value: T) {
        self.environment
            .push((key.to_string(), Mutex::new(Box::new(value))));
    }

    pub fn delete<K: Display>(&mut self, key: K) {
        let key = key.to_string();

        self.environment.retain(|(k, _)| &key != k);
    }
}

impl Default for TemplateEngine {
    fn default() -> Self {
        Self {
            base_path: Path::new(&"./views".to_string()).to_owned(),
            environment: Vec::new(),
            use_layout: true,
            layout_path: Path::new(&"layout".to_string()).to_owned(),
        }
    }
}

pub trait Render {
    fn render(
        &self,
        environment: &Vec<(String, Mutex<Box<dyn Serialize + Send>>)>,
    ) -> anyhow::Result<String>;
}

impl Render for String {
    fn render(
        &self,
        environment: &Vec<(String, Mutex<Box<dyn Serialize + Send>>)>,
    ) -> anyhow::Result<String> {
        let mut env = vec![];
        let lua = Lua::new();

        for (k, v) in environment {
            let value = v.lock().unwrap();
            let value = value.as_ref();

            let value = lua.to_value(&value).unwrap();
            env.push((k.to_owned(), value));
        }

        for (k, v) in env {
            lua.globals().set(k, v).expect("Unable to assign globals.")
        }

        lua.load("function show(v) if (v or '') == '' then data = '' end end")
            .exec()
            .unwrap();

        lua.load("function hide(v) if (v or '') ~= '' then data = '' end end")
            .exec()
            .unwrap();

        lua.load("function maybe(v, o) return v or o end")
            .exec()
            .unwrap();

        lua.load("function format(...) data = string.format(data, ...) end")
            .exec()
            .unwrap();

        lua.load("function each(k) local template = data; data = ''; for _, post in ipairs(k) do data = data .. template:gsub('%$([a-zA-Z_]+)', post) end end").exec().unwrap();

        render(self, lua)
    }
}

pub fn render(template: &String, lua: Lua) -> anyhow::Result<String> {
    let mut buffer = vec![];
    let mut rewriter = HtmlRewriter::new(
        Settings {
            element_content_handlers: vec![element!("lua", |el: &mut Element| {
                let start_location = el.source_location().bytes().end;
                let expression = el.get_attribute("code").unwrap_or("".to_string());
                el.remove();
                if let Some(handlers) = el.end_tag_handlers() {
                    let source = template.clone();
                    let e = expression.clone();
                    let lua = lua.clone();

                    handlers.push(Box::new(move |end| {
                        let end_location = end.source_location().bytes().start;
                        let html = source[start_location..end_location].to_string();

                        lua.globals().set("data", html).unwrap();

                        lua.load(&e)
                            .exec()
                            .expect(format!("Invalid Lua expression. {}", e).as_str());

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
