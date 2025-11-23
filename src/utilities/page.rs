use std::{collections::HashMap, env};

use mlua::Lua;
use regex::{Captures, Regex};
use rshtml::traits::RsHtml;

pub trait Render {
    fn render(&mut self, environment: Option<HashMap<String, String>>) -> Result<String, String>;
}

impl Render for Box<dyn RsHtml> {
    fn render(&mut self, environment: Option<HashMap<String, String>>) -> Result<String, String> {
        render(self, environment)
    }
}

struct Block {
    /// The lua expression inside the source match.
    expression: String,
    /// Stuff between the two matches.
    data: String,
    /// This is the first character index of the block.
    start: usize,
    /// This is the last character index of the block.
    end: usize,
}

pub fn render(page: &mut Box<dyn RsHtml>, environment: Option<HashMap<String, String>>) -> Result<String, String> {
    let html = match RsHtml::render(page.as_mut()) {
        Ok(s) => s,
        Err(_) => return Err("Unable to render page".to_string()),
    };

    let source_regex = Regex::new(r"<!--%(.+)%-->").unwrap();
    let captures: Vec<Captures> = source_regex.captures_iter(&html).collect();

    Ok(process(html.clone(), captures, environment)?)
}

fn process(source: String, captures: Vec<Captures>, enviornment: Option<HashMap<String, String>>) -> Result<String, String> {
    let mut text = source;

    if captures.len() % 2 != 0 {
        return Err(
            "Execution groups must be in the form of <!--% expression %--><!--% end %-->"
                .to_string(),
        );
    }

    let mut blocks = Vec::<Block>::new();
    let lua = Lua::new();

    for _ in 0..captures.len() / 2 {
        let (_, [expression]) = captures[0].extract();
        let start = captures[0].get_match().start();
        let end = captures[1].get_match().end();

        let data_beginning = captures[0].get_match().end();
        let data_end = captures[1].get_match().start();

        blocks.push(Block {
            expression: expression.to_string(),
            start,
            end,
            data: text[data_beginning..data_end].to_string(),
        });
    }

    for block in blocks {
        if let Some(ref e) = enviornment {
            for (key, value) in e.into_iter() {
                lua.globals().set(key.to_string(), value.to_string()).unwrap();
            }
        }

        lua.globals().set("data", block.data).unwrap();
        lua.load(block.expression).exec().unwrap();

        let data: String = lua.globals().get("data").unwrap();

        let mut left = text[0..block.start].to_string().to_owned();
        let right = &text[block.end..text.len()];

        left.push_str(&data);
        left.push_str(right);

        text = left;
    }

    Ok(text)
}
