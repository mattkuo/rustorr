use std::{fmt, str};
//use bencode::stream_tokenizer::Token;

pub enum Token {
    Int(i64),
    Str(Vec<u8>),
    StartDict,
    StartList,
    End,
}

impl fmt::Debug for Token {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            Token::Int(num) => write!(formatter, "Int({})", num),
            Token::Str(ref vec) => {
                let string = str::from_utf8(vec);
                if string.is_ok() {
                    write!(formatter, "Str({})", string.unwrap())
                } else {
                    write!(formatter, "ByteStr({:?})", vec)
                }
            }
            Token::StartDict => write!(formatter, "StartDict"),
            Token::StartList => write!(formatter, "StartVec"),
            Token::End => write!(formatter, "End"),
        }
    }
}

pub struct BencodeTokenizer<I> {
    iter: I,
}

impl<I: Iterator<Item = u8>> BencodeTokenizer<I> {
    pub fn new(reader: I) -> BencodeTokenizer<I> {
        BencodeTokenizer { iter: reader }
    }

    fn parse_int(&mut self) -> i64 {
        let mut result: i64 = 0;
        let mut current: char = self.iter.next().unwrap() as char;
        let is_negative: bool = current == '-';

        if is_negative {
            current = self.iter.next().unwrap() as char;
        }

        while current != 'e' {
            result = result * 10 + current.to_digit(10).unwrap() as i64;
            current = self.iter.next().unwrap() as char;
        }

        if is_negative {
            result *= -1;
        }

        result
    }

    fn parse_string(&mut self, first_n: char) -> Vec<u8> {
        let mut length: u32 = first_n.to_digit(10).unwrap();

        while let Some(c) = self.iter.next() {
            let c: char = c as char;
            if c.is_digit(10) {
                length = length * 10 + c.to_digit(10).unwrap();
            } else {
                // Iterator should consume ':' char
                break;
            }
        }

        if length == 0 {
            return Vec::new();
        }

        let mut result: Vec<u8> = Vec::with_capacity(length as usize);

        while result.len() < result.capacity() {
            let next_char: u8 = self.iter.next().unwrap();
            result.push(next_char);
        }

        result
    }
}

impl<I: Iterator<Item = u8>> Iterator for BencodeTokenizer<I> {
    type Item = Token;
    fn next(&mut self) -> Option<Token> {
        let next_char: char = match self.iter.next() {
            Some(c) => c as char,
            None => return None,
        };

        let result: Option<Token> = match next_char {
            'i' => Some(Token::Int(self.parse_int())),
            n @ '0'...'9' => Some(Token::Str(self.parse_string(n))),
            'l' => Some(Token::StartList),
            'd' => Some(Token::StartDict),
            'e' => Some(Token::End),
            _ => None,
        };

        result
    }
}
