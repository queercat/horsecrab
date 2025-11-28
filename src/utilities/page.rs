use std::fmt::Display;
use std::fs;
use std::path::{Path, PathBuf};

use erased_serde::Serialize;
use lawl::Lawl;

pub struct TemplateEngine {
    base_path: PathBuf,
    layout_path: PathBuf,
    use_layout: bool,
    lawl: Lawl,
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
        let mut rendered = self.lawl.render(&template).unwrap();

        if self.use_layout {
            self.lawl.insert(&"children", rendered);
            template = fs::read_to_string(self.get_path_to_template(&self.layout_path))?;
            rendered = self.lawl.render(&template).unwrap();
            self.lawl.remove(&"children");
        }

        Ok(rendered)
    }

    pub fn insert<T: Sync + Send + Serialize + 'static>(
        &mut self,
        key: &impl Display,
        value: T,
    ) -> Result<(), ()> {
        self.lawl.insert(&key.to_string(), value)
    }

    pub fn remove(&mut self, key: &impl Display) -> Result<(), ()> {
        self.lawl.remove(&key)
    }
}

impl Default for TemplateEngine {
    fn default() -> Self {
        Self {
            base_path: Path::new(&"./views".to_string()).to_owned(),
            use_layout: true,
            layout_path: Path::new(&"layout".to_string()).to_owned(),
            lawl: Default::default(),
        }
    }
}
