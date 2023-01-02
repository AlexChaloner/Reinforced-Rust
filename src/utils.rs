use std::io::{BufRead, Write};

pub fn prompt<R, W>(mut reader: R, mut writer: &mut W, question: &str) -> String
where
    R: BufRead,
    W: Write,
{
    write!(&mut writer, "{}", question).expect("Unable to write");
    let mut s = String::new();
    reader.read_line(&mut s).expect("Unable to read");
    s
}