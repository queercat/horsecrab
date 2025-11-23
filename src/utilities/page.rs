use std::{collections::HashMap, env, io::Cursor};

use html5ever::{
    ParseOpts, interface::ElemName, local_name, parse_document, parse_fragment, serialize, tendril::{Tendril, TendrilSink}
};
use markup5ever_rcdom::{Handle, Node, NodeData, RcDom, SerializableHandle};
use mlua::Lua;
use regex::{Captures, Regex};

pub trait Render {
    fn render(&mut self, environment: Option<&HashMap<String, String>>) -> Result<String, String>;
}

impl Render for String {
    fn render(&mut self, environment: Option<&HashMap<String, String>>) -> Result<String, String> {
        render(self.to_owned(), environment)
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

pub fn process(handle: &mut Handle) {
    let node = handle;

    match &node.data {
        NodeData::Element {
            name,
            attrs,
            template_contents,
            mathml_annotation_xml_integration_point,
        } => {
            if name.local.to_string() == "lua".to_string() {
            }
        }
        _ => (),
    }

    for (idx, child) in node.children.borrow_mut().iter_mut().enumerate() {
        process(child)
    }
}

pub fn render(
    template: String,
    environment: Option<&HashMap<String, String>>,
) -> Result<String, String> {
    let mut stream = Cursor::new(template);
    let opts = ParseOpts::default();
    let mut dom = parse_document(RcDom::default(), opts)
        .from_utf8()
        .read_from(&mut stream)
        .unwrap();

    process(&mut dom.document);

    let mut bytes = vec![];
    let document: SerializableHandle = dom.document.clone().into();
    serialize(&mut bytes, &document, Default::default()).unwrap();

    let text = String::from_utf8(bytes).unwrap();

    Ok(text)
}

// fn get_captures(source: &String) -> std::option::Option<regex::Captures<'_>> {
//     let source_regex = Regex::new(r"(?<start_token><!--%)|(?<end_token>%-->)").unwrap();
//     source_regex.captures(&source)
// }

// fn evaluate(source: &String, captures: Captures<'_>, environment: Option<&HashMap<String, String>>) -> Result<String, String> {
//     let mut text = source;

//     let lua = Lua::new();

//     if let Some(e) = environment {
//         for (key, value) in e.into_iter() {
//             lua.globals().set(key.to_string(), value.to_string()).unwrap();
//         }
//     }

//     lua.globals().set("data", block.data).unwrap();
//     lua.load(block.expression).exec().unwrap();

//     let data: String = lua.globals().get("data").unwrap();

//     let mut left = text[0..block.start].to_string().to_owned();
//     let right = &text[block.end..text.len()];

//     left.push_str(&data);
//     left.push_str(right);

//     text = left;

//     Ok(text)
// }
