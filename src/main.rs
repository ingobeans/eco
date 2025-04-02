use convert::*;
use std::{collections::HashMap, env::args};
mod convert;

fn get_selection_hashmaps() -> (
    HashMap<&'static str, fn(Vec<String>) -> Vec<u8>>,
    HashMap<&'static str, fn(Vec<u8>)>,
) {
    let mut hashmap_from: HashMap<&str, fn(Vec<String>) -> Vec<u8>> = HashMap::new();
    hashmap_from.insert("binary", parse_binary);

    hashmap_from.insert("decimal", parse_decimal);

    hashmap_from.insert("base64", parse_base64);
    hashmap_from.insert("64", parse_base64);

    hashmap_from.insert("hex", parse_hex);

    hashmap_from.insert("utf8", parse_utf8);
    hashmap_from.insert("text", parse_utf8);
    hashmap_from.insert("ascii", parse_utf8); // ascii is backwards compatibable with utf-8

    let mut hashmap_to: HashMap<&str, fn(Vec<u8>)> = HashMap::new();
    hashmap_to.insert("binary", to_binary);

    hashmap_to.insert("decimal", to_decimal);

    hashmap_to.insert("base64", to_base64);
    hashmap_to.insert("64", to_base64);

    hashmap_to.insert("hex", to_hex);

    hashmap_to.insert("raw", to_raw);

    hashmap_to.insert("utf8", to_utf8);
    hashmap_to.insert("text", to_utf8);
    hashmap_to.insert("ascii", to_ascii);

    (hashmap_from, hashmap_to)
}

/// Parses a selection argument, like "binary-hex", or "t-d" to corresponding functions of conversion
fn parse_selection(arg: String) -> Option<(fn(Vec<String>) -> Vec<u8>, fn(Vec<u8>))> {
    let (left, right) = arg.split_once('-')?;
    let (hashmap_from, hashmap_to) = get_selection_hashmaps();

    /// Function that searches a `hashmap` for any key that starts_with a specified `query`
    fn search_hashmap<'a, T>(hashmap: &'a HashMap<&'static str, T>, query: &str) -> Option<&'a T> {
        for (key, value) in hashmap.iter() {
            if key.starts_with(&query) {
                return Some(value);
            }
        }
        None
    }

    let from = search_hashmap(&hashmap_from, left)?;
    let to = search_hashmap(&hashmap_to, right)?;

    Some((*from, *to))
}

fn print_help() -> ! {
    // grab help message from readme.md
    let readme = include_str!("../readme.md").replace("`", "\"");
    let (_, mut text) = readme.split_once("## help message").unwrap();
    text = text.trim();
    println!("{text}");

    std::process::exit(1);
}

fn main() {
    let mut args = args();

    let Some(selection_arg) = args.nth(1) else {
        print_help()
    };
    let Some((from, to)) = parse_selection(selection_arg) else {
        print_help()
    };

    let args_vec = args.collect();
    let data = from(args_vec);
    //println!("{data:?}");
    to(data);
}
