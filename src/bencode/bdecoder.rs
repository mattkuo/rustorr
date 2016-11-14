use std::collections::HashMap;
use bencode::stream_tokenizer::BencodeTokenizer;
use bencode::stream_tokenizer::Token;

#[derive(Debug)]
pub enum Bencode {
    Dict(HashMap<String, Bencode>),
    Vector(Vec<Bencode>),
    Int(i64),
    Str(String),
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

    pub fn decode(&mut self) -> Bencode {
        if let Some(token) = self.tokenizer.next() {
            let value: Bencode = match token {
                Token::Int(num) => Bencode::Int(0),
                Token::Str(string) => Bencode::Str("hello".to_string()),
                Token::StartDict => self.decode_dict(),
                Token::StartVec => Bencode::Vector(Vec::new()),
                Token::Separator => Bencode::Int(0),
                Token::End => Bencode::Int(0),
            };

            return value;
        } else {
            return Bencode::Int(0);
        }
    }

    fn decode_dict(&mut self) -> Bencode {
        let mut map = HashMap::new();

        while let Some(token) = self.tokenizer.next() {
            let key = match token {
                Token::Str(string) => string,
                Token::End => return Bencode::Dict(map),
                _ => continue,
            };

            let value = match self.tokenizer.next() {
                Some(Token::Int(num)) => Bencode::Int(num),
                Some(Token::Str(string)) => Bencode::Str(string),
                Some(Token::End) => return Bencode::Dict(map),
                _ => continue,
            };

            map.insert(key, value);
        }

        return Bencode::Dict(map);
    }
}
