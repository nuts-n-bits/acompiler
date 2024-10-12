#![allow(dead_code)]

#[derive(Debug)]
struct TokenizerWorktable {
    cursor: usize,
    text: Vec<char>,
}

#[derive(Debug, PartialEq)]
pub enum Keyword {
    Let,
    Fn,
    If,
    Else,
    Then,
    Import,
    Pub,
    Module,
    Struct,
    Enum,
    Return,
    Require,
    External,
    In,
}

#[derive(Debug, PartialEq)]
pub enum Token {
    Identifier(String),
    Plus,
    PlusPlus,
    PlusEqual,
    Star,
    StarStar,
    Minus,
    MinusMinus,
    MinusEqual,
    Colon,
    ColonColon,
    Equal,
    EqualEqual,
    Dot,
    DotDot,
    ThinArrow,
    FatArrow,
    Lt,
    LtEqual,
    Gt,
    GtEqual,
    Bang,
    BangEqual,
    Lparen,
    Rparen,
    Lbracket,
    Rbracket,
    Lbrace,
    Rbrace,
    Comma,
    Slash,
    Keyword(Keyword),
    EOF,
}

impl<'a> TokenizerWorktable {
    fn new(program_text: &'a str) -> TokenizerWorktable {
        TokenizerWorktable {
            cursor: 0,
            text: program_text.chars().collect(),
        }
    }

    fn peek(&self, offset: usize) -> Option<char> {
        let index = self.cursor + offset;
        if index >= self.text.len() {
            return None;
        } else {
            return Some(self.text[index]);
        }
    }

    fn step(&mut self, offset: usize) {
        self.cursor += offset;
    }
}

pub fn tokenize(program_text: &str) -> Vec<Token> {
    let mut twt = TokenizerWorktable::new(program_text);
    let mut token_list: Vec<Token> = vec![];
    loop {
        let cur_char = match twt.peek(0) {
            Some(char) => char,
            None => {
                token_list.push(Token::EOF);
                break;
            }
        };
        match cur_char {
            '(' => {
                token_list.push(Token::Lparen);
                twt.step(1);
            }
            ')' => {
                token_list.push(Token::Rparen);
                twt.step(1);
            }
            '[' => {
                token_list.push(Token::Lbracket);
                twt.step(1);
            }
            ']' => {
                token_list.push(Token::Rbracket);
                twt.step(1);
            }
            '{' => {
                token_list.push(Token::Lbrace);
                twt.step(1);
            }
            '}' => {
                token_list.push(Token::Rbrace);
                twt.step(1);
            }
            ':' => match twt.peek(1) {
                Some(':') => {
                    token_list.push(Token::ColonColon);
                    twt.step(2);
                }
                _ => {
                    token_list.push(Token::Colon);
                    twt.step(1);
                }
            }
            ',' => {
                token_list.push(Token::Comma);
                twt.step(1);
            }
            '*' => match twt.peek(1) {
                Some('*') => {
                    token_list.push(Token::StarStar);
                    twt.step(1);
                }
                _ => {
                    token_list.push(Token::Star);
                    twt.step(1);
                }
            }
            '/' => {
                token_list.push(Token::Slash);
                twt.step(1);
            }
            '=' => match twt.peek(1) {
                Some('=') => {
                    token_list.push(Token::EqualEqual);
                    twt.step(2);
                }
                Some('>') => {
                    token_list.push(Token::FatArrow);
                    twt.step(2);
                }
                _ => {
                    token_list.push(Token::Equal);
                    twt.step(1);
                }
            }
            '!' => match twt.peek(1) {
                Some('=') => {
                    token_list.push(Token::BangEqual);
                    twt.step(2);
                }
                _ => {
                    token_list.push(Token::Equal);
                    twt.step(1);
                }
            }
            '-' => match twt.peek(1) {
                Some('>') => {
                    token_list.push(Token::ThinArrow);
                    twt.step(2);
                }
                Some('-') => {
                    token_list.push(Token::MinusMinus);
                    twt.step(2);
                }
                Some('=') => {
                    token_list.push(Token::MinusEqual);
                    twt.step(2);
                }
                _ => {
                    token_list.push(Token::Minus);
                    twt.step(1);
                }
            }
            '+' => match twt.peek(1) {
                Some('+') => {
                    token_list.push(Token::PlusPlus);
                    twt.step(2);
                }
                Some('=') => {
                    token_list.push(Token::PlusEqual);
                    twt.step(2);
                }
                _ => {
                    token_list.push(Token::Plus);
                    twt.step(1);
                }
            }
            '>' => {
                match twt.peek(1) {
                    Some('=') => {
                        token_list.push(Token::GtEqual);
                        twt.step(2);
                    }
                    _ => {
                        token_list.push(Token::Gt);
                        twt.step(1);
                    }
                }
            }
            '<' => {
                match twt.peek(1) {
                    Some('=') => {
                        token_list.push(Token::LtEqual);
                        twt.step(2);
                    }
                    _ => {
                        token_list.push(Token::Lt);
                        twt.step(1);
                    }
                }
            }
            ch if is_can_start_ident(ch) => {
                let mut ident_string = String::from(ch);
                loop {
                    twt.step(1);
                    match twt.peek(0) {
                        Some(ch) if is_ident(ch) => ident_string.push(ch),
                        _ => break,
                    }
                }
                match Keyword::from_ident(&ident_string) {
                    Some(keyword) => token_list.push(Token::Keyword(keyword)),
                    _ => token_list.push(Token::Identifier(ident_string)),
                }
            }
            ' ' => twt.step(1),
            '\t' => twt.step(1),
            '\n' => twt.step(1),
            '\r' => twt.step(1),
            _ => todo!()
        }
    }
    token_list
}




impl Keyword {
    fn from_ident(ident: &str) -> Option<Keyword> {
        match ident {
            "let" => Some(Keyword::Let),
            "fn" => Some(Keyword::Fn),
            "if" => Some(Keyword::If),
            "else" => Some(Keyword::Else),
            "then" => Some(Keyword::Then),
            "import" => Some(Keyword::Import),
            "pub" => Some(Keyword::Pub),
            "module" => Some(Keyword::Module),
            "struct" => Some(Keyword::Struct),
            "enum" => Some(Keyword::Enum),
            "return" => Some(Keyword::Return),
            "require" => Some(Keyword::Require),
            "external" => Some(Keyword::External),
            "in" => Some(Keyword::In),
            _ => None
        }
    }
}


fn is_can_start_ident(ch: char) -> bool {
    (ch >= 'a' && ch <= 'z') || (ch >= 'A' && ch <= 'Z') || ch == '_'
}

fn is_digit(ch: char) -> bool {
    ch >= '0' && ch <= '9'
}

fn is_ident(ch: char) -> bool {
    is_can_start_ident(ch) || is_digit(ch)
}