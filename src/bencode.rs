use std::collections::HashMap;
use std::collections::VecDeque;

enum Bencode {
    Dict(HashMap<String, Bencode>),
    Vector(Vec<Bencode>),
    Int(i64),
    Str(String),
}

#[derive(Debug)]
enum Token {
    StartDict,
    StartVec,
    End,
    Int(i64),
    Str(String),
}

struct BencodeTokenizer<I> {
    iter: I,
    location: VecDeque<Token>,
}

impl <I: Iterator<Item=u8>> BencodeTokenizer<I> {
    pub fn new(mut reader: I) -> BencodeTokenizer<I> {
        BencodeTokenizer {
            iter: reader,
            location: VecDeque::new(),
        }
    }

    fn parseInt(&mut self) -> i64 {
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

        return result;
    }

    fn parseStr(&mut self, first_n: char) -> String {
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
            return String::new();
        }
        
        let mut result: String = String::with_capacity(length as usize);

        while result.len() < result.capacity() {
            let next_char: char = self.iter.next().unwrap() as char;
            result.push(next_char);
        }

        return result;
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
            'i' => Some(Token::Int(self.parseInt())),
            n @ '0' ... '9' => Some(Token::Str(self.parseStr(n))),
            _ => None
        };

        return result;
    }
}

pub struct Bdecoder<I> {
    tokenizer: BencodeTokenizer<I>,
}

impl <I: Iterator<Item=u8>> Bdecoder<I> {
    pub fn new(mut iter: I) -> Bdecoder<I> {
        Bdecoder {
            tokenizer: BencodeTokenizer::new(iter),
        }
    }

    pub fn decode(&mut self) {
        print!("{:?}", self.tokenizer.next());
    }
}
