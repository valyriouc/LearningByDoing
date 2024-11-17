use std::env;
use crate::dom::{Node, NodeType};
use crate::html::Parser;

mod dom;
mod html;

fn main() {
    let args: Vec<String> = env::args().collect();
    let command = &args[1];
    match command.as_str() {
        "show" => show_dom_tree(),
        "help" => show_help(),
        _ => panic!("{}", format!("Unknown command: {command}"))
    }
}

fn show_dom_tree() {
    let html = r#"
        <html>
            <head>
                <title>Testing</title>
            </head>
            <body>
                <!-- This is a comment -->
                <h1>Title</h1>
                <div id="main" class="test">
                    <p>Hello <em>world</em>!</p>
                </div>
            </body>
        </html>
        "#;

    let root = Parser::parse(html.to_string());
    recursive_printing(root, 0);
}

fn recursive_printing(node: Node, depth: usize) {
    let spacer = (0..depth).map(|_| " ").collect::<String>();
    match node.node_type {
        NodeType::Text(content) => println!("{spacer}| {content}"),
        NodeType::Element(element) => {
            let tag_name = element.tag_name;
            println!("{spacer}| -> Tag: {tag_name}");
            for attr in element.attrs.iter() {
                let key = attr.0;
                let value = attr.1;
                println!("{spacer}|- Attr: {key} = {value}");
            }
            for child in node.children.into_iter() {
                recursive_printing(child, depth + 1);
            }
        }
        NodeType::Comment(content) => println!("{spacer}| Comment - {content}"),
        _ => print!("")
    }
}

fn show_help() {

}
