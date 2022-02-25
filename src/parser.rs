use logos::Logos;

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum Prefabs {
    None,
    Melee,
    Projectile,
    JumpPad,
    Stairs,
    HiM,
}

#[derive(Debug)]
pub struct Cell {
    height: i32,
    prefab: Prefabs,
}

impl Cell {
    pub fn height(&self) -> i32 {
        self.height
    }

    pub fn prefab(&self) -> Prefabs {
        self.prefab
    }
}

pub struct Pattern(pub Vec<Vec<Cell>>);

pub fn parse(source: String) -> Pattern {
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

    let (l, r) = filtered_tokens.split_at(16);

    for i in 0..16 {
        let mut buff = Vec::with_capacity(16);
        for j in 0..16 {
            let height = match l[i][j] {
                Tokens::Number(n) => n,
                Tokens::Prefab(n) if n == Prefabs::None => 0,
                _ => {
                    println!("Oh no an error");
                    std::process::exit(1);
                }
            };
            let prefab = match r[i][j] {
                Tokens::Prefab(n) => n,
                Tokens::Number(_) => Prefabs::None,
                Tokens::Error => {
                    println!("Oh no an error");
                    std::process::exit(1);
                }
            };

            buff.push(Cell {height, prefab});

        }
        pattern.0.push(buff)
    }

    pattern
}

#[derive(Logos, Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum Tokens {
    #[regex(r"\d|\(\d+\)", |lexer| {
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
