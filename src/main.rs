use std::env;
use std::process::Command;

fn is_cyrillic(line: &str) -> bool {
    if line.len() == 0 {
        return true;
    }
    let mut utf16 = [0; 2];
    let first = match line.chars().find(|x| x.is_alphabetic()) {
        Some(x) => x.encode_utf16(&mut utf16),
        None => &mut utf16,
    };
    if first.len() > 1 {
        return false;
    };
    if first[0] >= 1024 && first[0] <= 1105 {
        return true;
    }
    false
}

fn www_address(line: &str) -> String {
    let engine = match is_cyrillic(line) {
        true => "https://yandex.ru/search/?text=",
        false => "https://www.google.com/search?q=",
    };
    format!("{}{}", engine, line).to_string()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let line: String = args[1..].join(" ");
    Command::new("firefox")
        .args(&[www_address(&line)])
        .spawn()
        .unwrap();
}
