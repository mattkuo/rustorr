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

        let value: Option<Token> = match self.tokenizer.next() {
            Some(Token::Int(num)) => Bencode::Int(num),
            Some(Token::Str(string)) => Bencode::Str(string),
            Some(Token::StartDict) => self.decode_dict(),
            Some(Token::StartVec) => self.decode_vec(),
            Some(Token::End) => Bencode::Int(0),
            _ => Bencode::Int(0),
        };

        value
    }

    fn decode_dict(&mut self) -> Bencode {
        let mut map: HashMap<String, Bencode> = HashMap::new();

        while let Some(token) = self.tokenizer.next() {
            let key: String = match token {
                Token::Str(string) => string,
                Token::End => return Bencode::Dict(map),
                _ => continue,
            };

            let value: Bencode = match self.tokenizer.next() {
                Some(Token::Int(num)) => Bencode::Int(num),
                Some(Token::Str(string)) => Bencode::Str(string),
                Some(Token::StartDict) => self.decode_dict(),
                Some(Token::StartVec) => self.decode_vec(),
                Some(Token::End) => break,
                _ => continue,
            };

            map.insert(key, value);
        }

        Bencode::Dict(map)
    }

    fn decode_vec(&mut self) -> Bencode {
        let mut vec: Vec<Bencode> = Vec::new();

        while let Some(token) = self.tokenizer.next() {
            let val: Bencode = match token {
                Token::Int(int) => Bencode::Int(int),
                Token::Str(string) => Bencode::Str(string),
                Token::StartDict =>  self.decode_dict(),
                Token::StartVec => self.decode_vec(),
                Token::End => break,
            };

            vec.push(val);
        }

        Bencode::Vector(vec)
    }
}
