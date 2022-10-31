use std::{error::Error, fmt::Display};

use logos::Logos;

/// Representation of the prefabs available on the official editor
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum Prefabs {
    None,
    Melee,
    Projectile,
    JumpPad,
    Stairs,
    HiM,
}

impl Default for Prefabs {
    fn default() -> Self {
        Self::None
    }
}

impl Prefabs {
    /// Returns the text representation of a prefab as seen on the official editor
    pub fn short_name<'a>(&self) -> &'a str {
        match self {
            Prefabs::None => "0",
            Prefabs::Melee => "n",
            Prefabs::Projectile => "p",
            Prefabs::JumpPad => "J",
            Prefabs::Stairs => "s",
            Prefabs::HiM => "H",
        }
    }
}

/// Representation of an individual square on a cyber grind pattern
#[derive(Debug, Default, Clone, Copy)]
pub struct Cell {
    height: i32,
    prefab: Prefabs,
}

impl Cell {
    /// Gets the height of this cell
    pub fn height(&self) -> i32 {
        self.height
    }

    /// Gets the prefab of this cell
    pub fn prefab(&self) -> Prefabs {
        self.prefab
    }

    /// Checks if the cell's prefab is a `Prefabs::None` (`0`)
    pub fn is_none(&self) -> bool {
        self.prefab == Prefabs::None
    }
}

/// Representation of a cgp
#[derive(Debug)]
pub struct Pattern(pub Vec<Vec<Cell>>);

impl Pattern {
    /// Turns this struct back into a cgp
    pub fn to_pattern_string(&self) -> String {
        let pattern = &self.0;

        let mut height_buf = Vec::new();
        let mut prefab_buf = Vec::new();

        for i in 0..16 {
            for j in 0..16 {
                let cell = &pattern[i][j];

                let height_str = if cell.height >= 10 || cell.height.is_negative() {
                    format!("({})", cell.height)
                } else {
                    cell.height.to_string()
                };

                height_buf.push(height_str);
                prefab_buf.push(cell.prefab.short_name().to_owned());
            }

            height_buf.push("\n".to_owned());
            prefab_buf.push("\n".to_owned());
        }

        height_buf.push("\n".to_owned());

        [height_buf, prefab_buf]
            .concat()
            .into_iter()
            .collect::<String>()
            .trim()
            .to_owned()
    }
}

/// Tries to parse a string to a Pattern
pub fn parse(source: impl AsRef<str>) -> Result<Pattern, ParseError> {
    let source = source.as_ref();
    let lines = source.lines();
    let mut token_grid = Vec::new();
    let mut pattern = Pattern(Vec::new());

    for line in lines {
        let mut linebuf = Vec::new();
        let lexer = Tokens::lexer(line);

        lexer.for_each(|token| linebuf.push(token));
        token_grid.push(linebuf);
    }

    let filtered_tokens = token_grid
        .into_iter()
        .filter(|p| !p.is_empty())
        .collect::<Vec<_>>();

    if filtered_tokens.len() < 32 {
        return Err(ParseError("Input is not long enough to be a valid pattern.".into()));
    }

    let (l, r) = filtered_tokens.split_at(16);

    for i in 0..16 {
        let mut buff = Vec::with_capacity(16);
        for j in 0..16 {
            let height = match l[i][j] {
                Tokens::Number(n) => n,
                Tokens::Prefab(n) if n == Prefabs::None => 0,
                _ => {
                    return Err(ParseError("Invalid token when parsing numbers".to_string()))
                }
            };
            let prefab = match r[i][j] {
                Tokens::Prefab(n) => n,
                Tokens::Number(_) => Prefabs::None,
                Tokens::Error => {
                    return Err(ParseError("Invalid token when parsing prefabs".to_string()))
                }
            };

            buff.push(Cell { height, prefab });
        }
        pattern.0.push(buff)
    }

    Ok(pattern)
}

/// Generic error type for parsing failures
#[derive(Debug, Clone)]
pub struct ParseError(pub String);

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for ParseError {}

#[derive(Logos, Clone, Copy, Debug, PartialEq, PartialOrd)]
#[doc(hidden)]
pub enum Tokens {
    #[regex(r"\d|\(-?\d+\)", |lexer| {
        let slice = lexer.slice();

        if slice.starts_with('(') {
            let mut chars = slice.chars();
            chars.next();
            chars.next_back();
            chars.as_str().parse()

        } else {
            slice.parse()
        }
    })]
    Number(i32),

    #[regex("[a-zA-Z0]", |lexer| {
        match lexer.slice().chars().next().unwrap() {
            '0' => Some(Prefabs::None),
            'n' => Some(Prefabs::Melee),
            'p' => Some(Prefabs::Projectile),
            'J' => Some(Prefabs::JumpPad),
            's' => Some(Prefabs::Stairs),
            'H' => Some(Prefabs::HiM),
            _ => None
        }
    },
    priority = 2)]
    Prefab(Prefabs),

    #[error]
    Error,
}

#[cfg(test)]
mod test {
    use std::{fs::File, io::Write};

    use super::*;

    #[test]
    fn serde() {
        let src = include_str!("../example.cgp");
        let out = parse(src).unwrap().to_pattern_string();

        assert_eq!(src, &*out);
    }

    #[cfg(feature = "draw2d")]
    #[test]
    fn parser_draw2d() {
        use crate::draw2d::draw::Draw2d;

        let src = include_str!("../example.cgp");
        let data = Draw2d::draw(parse(src).unwrap());

        let bytes = &*data;
        let mut file = File::create("example.png").unwrap();
        file.write_all(bytes).unwrap();
    }
}
// cargo test --package cygrind-utils --lib -- parser::test::parser_draw2d --exact --nocapture
