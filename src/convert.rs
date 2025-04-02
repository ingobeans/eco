pub fn parse_hex(input: Vec<String>) -> Vec<u8> {
    let mut new = Vec::new();
    for word in input {
        let value = u8::from_str_radix(&word, 16).expect("bad hex");
        new.push(value);
    }
    new
}
pub fn parse_decimal(input: Vec<String>) -> Vec<u8> {
    let mut new = Vec::new();
    for word in input {
        let value = word.parse().expect("bad binary");
        new.push(value);
    }
    new
}
pub fn parse_binary(input: Vec<String>) -> Vec<u8> {
    let mut new = Vec::new();
    for word in input {
        let value = u8::from_str_radix(&word, 2).expect("bad binary");
        new.push(value);
    }
    new
}
pub fn parse_utf8(input: Vec<String>) -> Vec<u8> {
    let text = input.join(" ");
    text.as_bytes().into()
}

pub fn to_hex(input: Vec<u8>) {
    for value in input {
        print!("{value:x} ")
    }
    print!("\n")
}
pub fn to_decimal(input: Vec<u8>) {
    for value in input {
        print!("{value}")
    }
    print!("\n")
}
pub fn to_binary(input: Vec<u8>) {
    for value in input {
        let text = format!("{value:b} ");
        let padding = std::iter::repeat("0")
            .take(8 - text.len() + 1)
            .collect::<String>();
        print!("{padding}{text}");
    }
    print!("\n")
}
pub fn to_ascii(input: Vec<u8>) {
    for value in input {
        print!("{}", value as char);
    }
    print!("\n")
}
pub fn to_utf8(input: Vec<u8>) {
    let text = String::from_utf8_lossy(&input);
    print!("{text}\n")
}
