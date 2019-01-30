pub mod color;
pub mod material;
pub mod ray;
pub mod shape;
pub mod tracer;
pub mod vec;

pub fn delimited_int<T: ToString>(delim: char, value: T) -> String {
    let as_str = value.to_string();
    let mut iter = as_str.chars().rev().peekable();
    let mut delimited = String::new();
    let mut char_count = 0;
    while let Some(ch) = iter.next() {
        delimited.insert(0, ch);
        char_count += 1;
        if char_count % 3 == 0 && iter.peek().is_some() {
            delimited.insert(0, delim);
        }
    }
    delimited
}
