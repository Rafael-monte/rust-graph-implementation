use std::{str::{FromStr}};

pub fn char_to_string(c: &char) -> String {
    let mut buffer: [u8; 4] = [0; 4];
    let as_string = c.encode_utf8(&mut buffer);
    return String::from(as_string);
}


pub fn char_to_i32(c: &char) -> Result<i32, <i32 as FromStr>::Err>{
    let _as_string = char_to_string(c);
    return _as_string.parse::<i32>();
}