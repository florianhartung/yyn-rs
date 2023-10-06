#[derive(Debug, PartialEq)]
pub enum Keyword {
    Fun,
    Exit, // Exits the program with exit code 1
}

impl Keyword {
    pub fn try_from_str(s: &str) -> Option<Self> {
        use Keyword::*;

        match s {
            "fun" => Some(Fun),
            "exit" => Some(Exit),
            _ => None,
        }
    }
}


// TODO add location information (line:column)
#[derive(Debug, PartialEq)]
pub enum Token {
    Keyword(Keyword),
    // TODO will be identifiers at beginning
    Identifier(String),
    Number(u32),
    LeftBrace,
    RightBrace,
    LeftParentheses,
    RightParentheses,
    LeftSquareBracket,
    RightSquareBracket,
    NewLine,
}
