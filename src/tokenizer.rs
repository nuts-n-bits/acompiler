#![allow(dead_code)]

struct TokenizerWorktable {
    cursor: usize,
    text: Vec<char>,
}
enum Token {
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
    KeywordLet,
    KeywordFn,
    KeywordIf,
    KeywordElse,
    KeywordThen,
    KeywordImport,
    KeywordPub,
    KeywordModule,
    KeywordStruct,
    KeywordEnum,
    KeywordReturn,
    KeywordRequire,
    KeywordExternal,
    KeywordIn,
    EOF,
}

impl TokenizerWorktable {
    fn new(program_text: String) -> TokenizerWorktable {
        TokenizerWorktable {
            cursor: 0,
            text: program_text.chars().collect(),
        }
    }

    fn peek(&self, offset: usize) -> Option<char> {
        let index = self.cursor + offset;
        if index > self.text.len() {
            return None;
        } else {
            return Some(self.text[index]);
        }
    }

    fn step(&mut self, offset: usize) {
        self.cursor += offset;
    }
}

fn tokenize(program_text: String) -> Vec<Token> {
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
                let ident_string = String::from(ch);
                loop {
                    twt.step(1);
                    
                }
            }

        }
    }
    token_list
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