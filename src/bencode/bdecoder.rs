use std::collections::BTreeMap;
use bencode::stream_tokenizer::BencodeTokenizer;
use bencode::stream_tokenizer::Token;

#[derive(Debug)]
pub enum Bencode {
    Dict(BTreeMap<String, Bencode>),
    Vector(Vec<Bencode>),
    Int(i64),
    Str(String),
}

pub struct Bdecoder<I> {
    tokenizer: BencodeTokenizer<I>,
}

impl <I: Iterator<Item=u8>> Bdecoder<I> {
    pub fn new(iter: I) -> Bdecoder<I> {
        Bdecoder {
            tokenizer: BencodeTokenizer::new(iter),
        }
    }

    pub fn decode(&mut self) -> Bencode {

        // TODO: Error checking
        let token: Token = self.tokenizer.next().unwrap();
        let bencode_option: Option<Bencode>  = self.token_to_bencode(token);

        // TODO: return error on Token::End
        bencode_option.unwrap()
    }

    fn decode_dict(&mut self) -> Bencode {
        let mut map: BTreeMap<String, Bencode> = BTreeMap::new();

        while let Some(token) = self.tokenizer.next() {
            let key: String = match token {
                Token::Str(string) => string,
                Token::End => return Bencode::Dict(map),
                _ => continue,
            };

            // TODO: Error checking here
            let token: Token = self.tokenizer.next().unwrap();
            let bencode_option: Option<Bencode>  = self.token_to_bencode(token);

            if let Some(bencode) = bencode_option {
                map.insert(key, bencode);
            } else {
                break;
            }
        }

        Bencode::Dict(map)
    }

    fn decode_vec(&mut self) -> Bencode {
        let mut vec: Vec<Bencode> = Vec::new();

        while let Some(token) = self.tokenizer.next() {
            let bencode_option: Option<Bencode> = self.token_to_bencode(token);

            if let Some(bencode) = bencode_option {
                vec.push(bencode);
            } else {
                break;
            }
        }

        Bencode::Vector(vec)
    }

    fn token_to_bencode(&mut self, token: Token) -> Option<Bencode> {
        let bencode: Bencode = match token {
            Token::Int(int) => Bencode::Int(int),
            Token::Str(string) => Bencode::Str(string),
            Token::StartDict =>  self.decode_dict(),
            Token::StartVec => self.decode_vec(),
            Token::End => return None,
        };

        Some(bencode)
    }
}
