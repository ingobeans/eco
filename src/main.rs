use std::{collections::HashMap, env::args};

fn parse_hex(input: Vec<String>) -> Vec<u8> {
    let mut new = Vec::new();
    for word in input {
        let value = u8::from_str_radix(&word, 16).expect("bad hex");
        new.push(value);
    }
    new
}
fn parse_decimal(input: Vec<String>) -> Vec<u8> {
    let mut new = Vec::new();
    for word in input {
        let value = word.parse().expect("bad binary");
        new.push(value);
    }
    new
}
fn parse_binary(input: Vec<String>) -> Vec<u8> {
    let mut new = Vec::new();
    for word in input {
        let value = u8::from_str_radix(&word, 2).expect("bad binary");
        new.push(value);
    }
    new
}
fn parse_utf8(input: Vec<String>) -> Vec<u8> {
    let text = input.join(" ");
    text.as_bytes().into()
}

fn to_hex(input: Vec<u8>) {
    for value in input {
        print!("{value:x} ")
    }
    print!("\n")
}
fn to_decimal(input: Vec<u8>) {
    for value in input {
        print!("{value}")
    }
    print!("\n")
}
fn to_binary(input: Vec<u8>) {
    for value in input {
        let text = format!("{value:b} ");
        let padding = std::iter::repeat("0")
            .take(8 - text.len() + 1)
            .collect::<String>();
        print!("{padding}{text}");
    }
    print!("\n")
}
fn to_ascii(input: Vec<u8>) {
    for value in input {
        print!("{}", value as char);
    }
    print!("\n")
}
fn to_utf8(input: Vec<u8>) {
    let text = String::from_utf8_lossy(&input);
    print!("{text}\n")
}

fn get_selection_hashmaps() -> (
    HashMap<&'static str, fn(Vec<String>) -> Vec<u8>>,
    HashMap<&'static str, fn(Vec<u8>)>,
) {
    let mut hashmap_from: HashMap<&str, fn(Vec<String>) -> Vec<u8>> = HashMap::new();
    hashmap_from.insert("binary", parse_binary);
    hashmap_from.insert("decimal", parse_decimal);
    hashmap_from.insert("hex", parse_hex);
    hashmap_from.insert("utf8", parse_utf8);
    hashmap_from.insert("text", parse_utf8);

    let mut hashmap_to: HashMap<&str, fn(Vec<u8>)> = HashMap::new();
    hashmap_to.insert("binary", to_binary);
    hashmap_to.insert("decimal", to_decimal);
    hashmap_to.insert("hex", to_hex);
    hashmap_to.insert("utf8", to_utf8);
    hashmap_to.insert("text", to_utf8);

    (hashmap_from, hashmap_to)
}

fn parse_selection(arg: String) -> Option<(fn(Vec<String>) -> Vec<u8>, fn(Vec<u8>))> {
    let (left, right) = arg.split_once('-')?;
    let (hashmap_from, hashmap_to) = get_selection_hashmaps();

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

fn main() {
    let mut args = args();

    let from: fn(Vec<String>) -> Vec<u8>;
    let to: fn(Vec<u8>);

    let selection_arg = args.nth(1).expect("incorrect usage");
    (from, to) = parse_selection(selection_arg).expect("invalid selection");

    let args_vec = args.collect();
    let data = from(args_vec);
    //println!("{data:?}");
    to(data);
}
