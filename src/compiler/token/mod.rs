#[derive(Debug, PartialEq)]
pub enum Keyword {
    Fun,
    Exit69, // Exits the program with exit code 69
}

impl Keyword {
    pub fn try_from_str(s: &str) -> Option<Self> {
        use Keyword::*;

        match s {
            "fun" => Some(Fun),
            "exit69" => Some(Exit69),
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
    LeftBrace,
    RightBrace,
    LeftParentheses,
    RightParentheses,
    LeftSquareBracket,
    RightSquareBracket,
    NewLine,
}