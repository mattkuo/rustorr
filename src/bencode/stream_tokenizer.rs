use std::{fmt, str};
use self::Token::*;

pub enum Token {
    Int(i64),
    Str(Vec<u8>),
    StartDict,
    StartVec,
    End,
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            Int(num) => write!(f, "Int({})", num),
            Str(ref vec) => {
                let string = str::from_utf8(vec);
                if string.is_ok() {
                    return write!(f, "Str({})", string.unwrap());
                } else {
                    return write!(f, "ByteStr({:?})", vec);
                }
            },
            StartDict => write!(f, "StartDict"),
            StartVec => write!(f, "StartVec"),
            End => write!(f, "End"),
        }
    }
}

pub struct BencodeTokenizer<I> {
    iter: I,
}

impl <I: Iterator<Item=u8>> BencodeTokenizer<I> {
    pub fn new(reader: I) -> BencodeTokenizer<I> {
        BencodeTokenizer {
            iter: reader,
        }
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

impl <I: Iterator<Item=u8>> Iterator for BencodeTokenizer<I> {
    type Item = Token;
    fn next(&mut self) -> Option<Token> {
        let next_char: char = match self.iter.next() {
            Some(c) => c as char,
            None => return None
        };

        let result: Option<Token> = match next_char {
            'i' => Some(Int(self.parse_int())),
            n @ '0' ... '9' => Some(Str(self.parse_string(n))),
            'l' => Some(StartVec),
            'd' => Some(StartDict),
            'e' => Some(End),
            _ => None
        };

        println!("{:?}", result);

        result
    }
}
