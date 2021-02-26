// NFA -> DFA Conversion
// Regex to Determinstic Finite atomata
// Tradeoffs between speed and space
// DFA faster less compact
// NFAs slower more conscise
#![feature(proc_macro_hygiene)]
extern crate plex;
use core::panic;

use plex::lexer;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Integer(i64),
    Ident(String),

    Slf,
    Comment,
    Whitespace,

    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    Colon,
    Semicolon,
    Comma,
    LeftAssign,
    RightAssign,
    Add,
    Sub,
    Mult,
    Divide,
    Tild,
    LT,
    LTEQ,
    GT,
    GTEQ,

    TypeID(Type),
    Bool(bool),
    Class,
    Else,
    Fi,
    If,
    In,
    Inherits,
    IsVoid,
    Let,
    Loop,
    Pool,
    Then,
    While,
    Case,
    Esac,
    New,
    Of,
    Not,
    Dispatch,

    Error(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    String,
    Int,
}

lexer! {
    fn take_token(token: 'a) -> Token;

    r#"--[^\n]*"# => Token::Comment,
    r#"[*][^]*[*]"# => Token::Comment,

    r#"[\n\f\r\t\v ]+"# => Token::Whitespace,
    r#"{"# => Token::LeftBracket,
    r#"}"# => Token::RightBracket,
    r#"\("# => Token::LeftParen,
    r#"\)"# => Token::RightParen,
    r#":"# => Token::Colon,
    r#";"# => Token::Semicolon,
    r#","# => Token::Comma,
    r#"<-"# => Token::LeftAssign,
    r#"=>"# => Token::RightAssign,
    r#"\+"# => Token::Add,
    r#"-"# => Token::Sub,
    r#"\*"# => Token::Mult,
    r#"/"# => Token::Divide,

    r#"[cC][lL][aA][sS][sS]"# => Token::Class,
    r#"[eE][lL][sS][eE]"# => Token::Else,
    r#"[fF][iI]"# => Token::Fi,
    r#"[iI][fF]"# => Token::If,
    r#"[iI][nN]"# => Token::In,
    r#"[iI][nN][hH][eE][rR][iI][tT][sS]"# => Token::Inherits,
    r#"[iI][sS][vV][oO][iI][dD]"# => Token::IsVoid,
    r#"[lL][eE][tT]"# => Token::Let,
    r#"[lL][oO][oO][pP]"# => Token::Loop,
    r#"[pP][oO][oO][lL]"# => Token::Pool,
    r#"[tT][hH][eE][nN]"# => Token::Then,
    r#"[wW][hH][iI][lL][eE]"# => Token::While,
    r#"[cC][aA][sS][eE]"# => Token::Case,
    r#"[eE][sS][aA][cC]"# => Token::Esac,
    r#"[nN][eE][wW]"# => Token::New,
    r#"[oO][fF]"# => Token::Of,
    r#"[nN][oO][tT]"# => Token::Not,

    r#"true"# => Token::Bool(true),
    r#"false"# => Token::Bool(false),

    r#"[0-9]+"# => {
        if let Ok(i) = token.parse() {
            Token::Integer(i)
        } else {
            Token::Error(format!("Out of range: {}", token))
        }
    }

    r#""([^]*(\\\n)?)*""# => Token::TypeID(Type::String),
    r#""([a-zA-Z 0-9]*(\\\n)?)*"# => Token::Error(format!("EOF in String: {}", token.to_owned())),

    r#"[a-zA-Z_0-9]+"# => {
        match token {
            "self" | "SELF_TYPE" => Token::Slf,
            _ => Token::Ident(token.to_owned()),
        }
    }
    "\\."=> Token::Dispatch,
    "." => {
        Token::Error(token.to_owned())
    }
}

pub struct Lexer<'a> {
    original: &'a str,
    remaining: &'a str,
}

impl<'a> Lexer<'a> {
    pub fn new(s: &'a str) -> Self {
        Self {
            original: s,
            remaining: s,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Span {
    pub lo: usize,
    pub hi: usize,
}

impl<'a> Iterator for Lexer<'a> {
    type Item = (Token, Span);
    fn next(&mut self) -> Option<(Token, Span)> {
        loop {
            let (tok, span) = if let Some((tok, new_remaining)) = take_token(self.remaining) {
                let lo = self.original.len() - self.remaining.len();
                let hi = self.original.len() - new_remaining.len();
                self.remaining = new_remaining;
                (tok, Span { lo, hi })
            } else {
                return None;
            };
            match tok {
                Token::Whitespace | Token::Comment => {
                    continue;
                }
                tok => {
                    return Some((tok, span));
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    static code: &str = r#"
--This is a comment
* More comments *
* 
One More comment 
*
class List inherits A2I {
    item: Object;
    next: List;

    init(i: String, n: Object): List {
        
        {
            item <- i;
            next <- n;
            self
        }
    };

    flatten(): String {
        let string: String <-
            case item of
                i: Int => i2a(i);
                s: String => s;
                o: Object =>  { abort(); ""; };
            esac
        in
            if (isvoid next) then
                string
            else
                string.concat(next.flatten())
            fi
    };
};

class Main inherits IO {
    main(): Object {
        let hello: String <- "Hello ",
            world: String <- "World!",
            i: int <- 43,
            newline: String <- "\n",
            nil: List,
            list: List <- 
                (new List).init(hello,
                    (new List).init(world, 
                        (new List).init(42,
                            (new List).init(newline, nil))))
        in
            out_string(list.flatten())
    };
};"#;

    #[test]
    fn test_comments() {
        let lex = Lexer::new(&code);
        let expected_tokens = vec![Token::Class];
        for (tok, span) in lex {
            match tok {
                Token::Error(err) => self::panic!("bad token"),
                _ => println!("{:?}", tok),
            }
        }
    }

    static eof_code: &str = r#"
"This is an EOF string"#;

    #[test]
    fn test_eof_in_string() {
        let lex = Lexer::new(eof_code);
        for (tok, span) in lex {
            assert_eq!(
                Token::Error("EOF in String: \"This is an EOF string".to_string()),
                tok
            )
        }
    }
}
