#![allow(dead_code)]

/// Validates a `.cgp` file, returning () or the ParseError
pub fn validate<S: AsRef<str>>(s: S) -> Result<(), parser::ParseError> {
    parser::parse(s)?;
    
    Ok(())
}

#[cfg(feature = "draw2d")]
pub mod draw2d;

/// Parser for the ULTRAKILL cyber grind pattern (cgp) format
pub mod parser;

/// Utilities for drawing patterns (extracting a colour from height information)
pub mod util;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_validate() {
        validate(include_str! ("../example.cgp")).unwrap();
    }
}
