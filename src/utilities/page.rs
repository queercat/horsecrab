use mlua::Lua;
use regex::{CaptureMatches, Captures, Regex};
use rshtml::traits::RsHtml;

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

pub fn render(page: &mut Box<dyn RsHtml>) -> Result<String, String> {
    let html = match page.render() {
        Ok(s) => s,
        Err(_) => return Err("Unable to render page".to_string()),
    };

    let source_regex = Regex::new(r"<!--%(.+)%-->").unwrap();
    let captures: Vec<Captures> = source_regex.captures_iter(&html).collect();

    Ok(process(html.clone(), captures)?)
}

fn process(source: String, captures: Vec<Captures>) -> Result<String, String> {
    let mut text = source;

    if captures.len() % 2 != 0 {
        return Err(
            "Execution groups must be in the form of <!--% expression %--><!--% end %-->"
                .to_string(),
        );
    }

    let mut blocks = Vec::<Block>::new();

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
        let lua = Lua::new();
        lua.globals().set("data", block.data).unwrap();
        lua.load(block.expression).exec().unwrap();

        let data: String = lua.globals().get("data").unwrap();

        text.push_str(&data);
    }

    Ok(text)
}
