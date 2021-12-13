use crate::cgp::*;

#[test]
fn parser() {
    let src = include_str!("test.cgp");
    let pattern = parser::parse(src.to_string());
    drawing::draw(pattern);
}
