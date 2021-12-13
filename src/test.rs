use std::{fs::File, io::Write};

use crate::cgp::*;

#[test]
fn parser() {
    let src = include_str!("../example.cgp");
    let data = drawing::draw(parser::parse(src.to_string()));

    let bytes = data.as_slice();
    let mut file = File::create("example.png").unwrap();
    file.write_all(bytes).unwrap();
}
