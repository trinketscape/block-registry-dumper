use std::fs::File;
use std::path::Path;
use std::io::{Read, Write};
use jzon::{JsonValue, object};
use tree_sitter::{Parser, Query, QueryCursor, StreamingIteratorMut};

fn read_file(path: &Path) -> std::io::Result<String> {
    let mut file = File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

/// Crudely matches for chained function calls within the BlockProperties constructor
/// and returns whatever's been the brackets.
fn extract_from_chain<'a>(prefix: &'static str, call: &'a str) -> Option<&'a str> {
    match call.find(prefix) {
        Some(enum_start_idx) => {
            let enum_end_idx = enum_start_idx + prefix.len(); // SoundType.GRAVEL
            let sub = &call[enum_end_idx..];

            match sub.find(")") {
                Some(closing_bracket_index) => {
                    return Some(&sub[..closing_bracket_index]);
                }
                None => None,
            }
        }
        None => None,
    }
}

fn main() {
    let path = Path::new("./Blocks.java");
    let content = read_file(&path).expect("failed to read `Blocks.java` - is it in the current working directory?");

    // Create parser

    let mut parser = Parser::new();
    let language = tree_sitter_java::LANGUAGE;

    parser
        .set_language(&language.into())
        .expect("failed to load java parser");

    let tree = parser.parse(content.clone(), None).unwrap();
    let root = tree.root_node();

    // the tree sitter playground is available online here for testing this! -> https://tree-sitter.github.io/tree-sitter/7-playground.html#query
    // (or just use its cli)
    let query_source = r#"
(variable_declarator
    (method_invocation
        (argument_list (string_literal)@block_name (method_reference)@constructor (method_invocation)@block_behaviour) @register_args) @register_fn) @block_name_constant

(variable_declarator
    (method_invocation
        (argument_list (string_literal)@block_name (method_invocation)@block_behaviour) @register_args) @register_fn) @block_name_constant
    "#;

    // Prepare query

    let mut cursor = QueryCursor::new();
    let mut json_dict = object! {};
    let query = Query::new(&language.into(), query_source).unwrap();
    let capture_names = query.capture_names();
    let matches = cursor.matches(&query, root, content.as_bytes());

    // Run query

    matches.for_each_mut(|query_match| {
        let new_objs = for_query_match(&content, capture_names, query_match);

        for (key, val) in new_objs.entries() {
            json_dict[key] = val.clone();
        }
    });

    // Output to json file

    let mut output_file = File::create("./block_registry_out.json").unwrap();
    let json_out = jzon::stringify_pretty(json_dict.clone(), 2);

    output_file
        .write(json_out.as_bytes())
        .expect("Failed to write file");

    println!("output {} block data entries!", json_dict.len());
}

fn for_query_match(
    content: &String,
    capture_names: &[&str],
    query_match: &mut tree_sitter::QueryMatch<'_, '_>,
) -> JsonValue {
    let mut json_objs = object! {};

    for capture in query_match.captures {
        //println!("expression: {}", capture.node.to_sexp());

        let capture_tag = capture_names[capture.index as usize];

        //let text = &content[capture.node.start_byte()..capture.node.end_byte()];
        // println!("Register Args {name} => {text}");

        match capture_tag {
            "register_args" => {
                let walking_stick = &mut capture.node.walk();
                // The child at index zero should be the function's bracket, so we skip it.

                let block_id = capture
                    .node
                    .child(1)
                    .expect("missing block id")
                    .child(1)
                    .expect("missing block id's string content");

                let block_id_text = &content[block_id.start_byte()..block_id.end_byte()];

                for param_node in capture.node.children(walking_stick) {
                    let text = &content[param_node.start_byte()..param_node.end_byte()];

                    if !text.starts_with("BlockBehaviour") {
                        continue;
                    }

                    //println!("cc: {} {}", capture.node.child_count(), text);

                    let mut block_data = jzon::JsonValue::new_object();

                    match extract_from_chain("SoundType.", text) {
                        Some(sound_type) => {
                            println!(
                                "{}'s sound type = {} [source: {text}]",
                                block_id_text, sound_type
                            );
                            block_data["soundType"] = sound_type.into();
                        }
                        None => println!("{} has no sound type", block_id_text),
                    }

                    match extract_from_chain(".explosionResistance(", text) {
                        Some(resistance) => {
                            let resistance_num: f32 = resistance
                                .replace("F", "")
                                .parse::<f32>()
                                .expect(&format!("{block_id_text}.explosionResistance(>>{resistance}<<)\n^ failed to parse this number!"));
                            
                            block_data["explosionResistance"] = resistance_num.into();
                        }
                        None => println!("{} has no explosion resistance", block_id_text),
                    }

                    if text.contains(".instabreak") { block_data["instabreak"] = true.into(); }
                    if text.contains(".noCollision") { block_data["noCollision"] = true.into(); }

                    if block_data.len() > 0 {
                        json_objs[block_id_text] = block_data;
                    }
                }
            }
            _ => {}
        };
    }

    return json_objs;
}
