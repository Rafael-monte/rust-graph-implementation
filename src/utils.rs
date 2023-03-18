pub fn char_to_string(c: &char) -> String {
    let mut buffer: [u8; 4] = [0; 4];
    let as_string = c.encode_utf8(&mut buffer);
    return String::from(as_string);
}