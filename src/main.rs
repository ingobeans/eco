use std::env::args;

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

fn parse_selection(arg: String) -> Option<(fn(Vec<String>) -> Vec<u8>, fn(Vec<u8>))> {
    let (left, right) = arg.split_once('-')?;

    let from = match left {
        "b" | "binary" => parse_binary,
        "d" | "decimal" => parse_decimal,
        "h" | "hex" => parse_hex,
        "u" | "t" | "utf8" | "text" => parse_utf8,
        _ => panic!(),
    };

    let to = match right {
        "b" | "binary" => to_binary,
        "d" | "decimal" => to_decimal,
        "h" | "hex" => to_hex,
        "a" | "ascii" => to_ascii,
        "u" | "t" | "utf8" | "text" => to_utf8,
        _ => panic!(),
    };

    Some((from, to))
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
