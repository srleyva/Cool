// NFA -> DFA Conversion
// Regex to Determinstic Finite atomata
// Tradeoffs between speed and space
// DFA faster less compact
// NFAs slower more conscise
#![feature(proc_macro_hygiene)]
extern crate plex;
use plex::lexer;

#[derive(Debug, Clone)]
pub enum Token {
    Integer(i64),
    Ident(String),

    slf,
    Str,
    Comments,
    Whitespace,

    Class,
    Else,
    False,
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
    True,
}

lexer! {
    fn take_token(token: 'a) -> Token<'a>;

    r#"[0-9]+"# => {
        if let Ok(i) = text.parse() {
            Token::Integer(i)
        } else {
            panic!("integer {} out of range")
        }
    }

    r#"[a-zA-Z_0-9]+"# = {
        let ident = text.to_owned();
        match ident {
            "self" | "SELF_TYPE" => Token::Slf,
            _ => Token::Ident(ident),
        }
    }

    r#""([a-zA-Z 0-9]*(\\\n)?)*""# => Token::Str;
    r#"--[a-zA-z0-9 ]*\n"# => Token::Comment;
    r#"[\n\f\r\t\v]+"# => Token::Whitespace;

    r#"[cC][lL][aA][sS][sS]"# => Token::Class;
    r#"[eE][lL][sS][eE]"# => Token::Else;
    r#"[fF][aA][lL][sS][eE]"# => Token::False;
    r#"[fF][iI]"# => Token::Fi;
    r#"[iI][fF]"# => Token::If;
    r#"[iI][nN]"# => Token::In;
    r#"[iI][nN][hH][eE][rR][iI][tT][sS]"# => Token::Inherits;
    r#"[iI][sS][vV][oO][iI][dD]"# => Token::IsVoid;
    r#"[lL][eE][tT]"# => Token::Let;
    r#"[lL][oO][oO][pP]"# => Token::Loop;
    r#"[pP][oO][oO][lL]"# => Token::Pool;
    r#"[tT][hH][eE][nN]"# => Token::Then;
    r#"[wW][hH][iI][lL][eE]"# => Token::While;
    r#"[cC][aA][sS][eE]"# => Token::Case;
    r#"[eE][sS][aA][cC]"# => Token::Esac;
    r#"[nN][eE][wW]"# => Token::New;
    r#"[oO][fF]"# => Token::Of;
    r#"[nN][oO][tT]"# => Token::Not;
    r#"[tT][rR][uU][eE]"# => Token::True;
}

pub struct Lexer {
    original: &'a str,
    remaining: &'a str,
}

impl<'a> Lexer<'a> {
    pub fn new(s: &'a str) -> Self<'a> {
        Self {
            original: s,
            remaining: s,
        }
    }
}
