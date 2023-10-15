#[derive(Debug, PartialEq)]
pub enum Keyword {
    Fun,
    Exit, // Exits the program with exit code 1
    Int,
}

impl Keyword {
    pub fn try_from_str(s: &str) -> Option<Self> {
        use Keyword::*;

        match s {
            "fun" => Some(Fun),
            "exit" => Some(Exit),
            "int" => Some(Int),
            _ => None,
        }
    }
}


// TODO add location information (line:column)
#[derive(Debug, PartialEq)]
pub enum Token {
    Keyword(Keyword),
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
