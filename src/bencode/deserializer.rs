use std::collections::BTreeMap;
use bencode::stream_tokenizer::BencodeTokenizer;
use bencode::stream_tokenizer::Token;
use bencode::bencode::Bencode;

pub struct BencodeDeserializer<I> {
    tokenizer: BencodeTokenizer<I>,
}

impl <I: Iterator<Item=u8>> BencodeDeserializer<I> {
    pub fn new(iter: I) -> BencodeDeserializer<I> {
        BencodeDeserializer {
            tokenizer: BencodeTokenizer::new(iter),
        }
    }

    pub fn deserialize(&mut self) -> Bencode {

        // TODO: Error checking
        let token: Token = self.tokenizer.next().unwrap();
        let bencode_option: Option<Bencode>  = self.token_to_bencode(token);

        // TODO: return error on Token::End
        bencode_option.unwrap()
    }

    fn deserialize_dict(&mut self) -> Bencode {
        let mut map: BTreeMap<String, Bencode> = BTreeMap::new();

        while let Some(token) = self.tokenizer.next() {
            let key: String = match token {
                Token::Str(string) => String::from_utf8(string).unwrap(),
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

    fn deserialize_list(&mut self) -> Bencode {
        let mut vec: Vec<Bencode> = Vec::new();

        while let Some(token) = self.tokenizer.next() {
            let bencode_option: Option<Bencode> = self.token_to_bencode(token);

            if let Some(bencode) = bencode_option {
                vec.push(bencode);
            } else {
                break;
            }
        }

        Bencode::List(vec)
    }

    fn token_to_bencode(&mut self, token: Token) -> Option<Bencode> {
        let bencode: Bencode = match token {
            Token::Int(int) => Bencode::Int(int),
            Token::Str(string) => Bencode::Str(string),
            Token::StartDict =>  self.deserialize_dict(),
            Token::StartList => self.deserialize_list(),
            Token::End => return None,
        };

        Some(bencode)
    }
}
