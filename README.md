# cygrind-utils

## A utility crate for handling, parsing, and drawing cybergrind patterns from ULTRAKILL

This crate can go from an unparsed `.cgp` file to a `.png` byte buffer in around a hundredth of a second

## Usage

```rs
use cygrind_utils::cgp::*;

fn main() {
    // Make sure that this is a String and not an &str 
    // An &str is used for demonstrative purposes (i asked nicely don't worry)
    let src = include_str!("example.cgp");
    let data = drawing::draw(parser::parse(src.to_string()));

    // woo a png buffer
    let mut bytes = data.as_slice(); 
}
```

## Bench comparisons

|oh|wait|there|are|none|
|---|---|-----|---|----|

<br>

Times:

- Parsing: `200 micros`

- Drawing: `11ms`

- File-ing: `Idk it takes a while IO is really slow (150ms)`
