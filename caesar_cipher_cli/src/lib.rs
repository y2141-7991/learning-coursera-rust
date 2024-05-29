pub fn encrypt(text: &str, shift: u8) -> String {
    let mut result = String::new();
    for c in text.chars() {
        if c.is_ascii_alphabetic() {
            let base = if c.is_ascii_lowercase() {b'a'} else {b'A'};
            let offset = (c as u8 - base + shift) % 26;
            println!("{} {} {}", c as u8, base, shift);
            result.push((base + offset) as char);
        }
    }
    result
}

pub fn decrypt(text: &str, shift: u8) -> String {
    encrypt(text, 26 - shift)
}