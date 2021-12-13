use std::{fs::File, io::Write};

use crate::cgp::*;

#[test]
fn parser() {
    let src = include_str!("test.cgp");
    let pattern = parser::parse(src.to_string());
    let data = drawing::draw(pattern);

    let bytes = data.as_slice();
    let mut file = File::create("test.png").unwrap();
    file.write_all(bytes).unwrap();
}
