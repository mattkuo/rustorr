extern crate reqwest;

use std::fmt::Write;
use std::collections::BTreeMap;
use bencode::bencode::Bencode;

pub struct BencodeSerializer {
}

impl BencodeSerializer {

    pub fn new() -> BencodeSerializer {
        BencodeSerializer {}
    }

    pub fn serialize(&self, bencoded: &Bencode) -> String {
        // TODO: implement using only one string allocation to prevent space being allocated over and over again.
        let result: String = match bencoded {
            &Bencode::Dict(ref dict) => self.serialize_dict(dict),
            &Bencode::List(ref list) => self.serialize_list(list),
            &Bencode::Int(int) => self.serialize_int(int),
            &Bencode::Str(ref string) => self.serialize_string(string) 
        };

        result
    }

    fn serialize_dict(&self, dict: &BTreeMap<String, Bencode>) -> String {
        let mut result = String::new();

        write!(&mut result, "d{}", dict.len()).unwrap();
        for (key, value) in dict.iter() {
            write!(&mut result, "{}{}", key, self.serialize(value)).unwrap();
        }
        write!(&mut result, "e").unwrap();

        result
    }

    fn serialize_list(&self, list: &[Bencode]) -> String {
        let mut result = String::new();

        write!(&mut result, "l{}", list.len()).unwrap();
        for value in list.iter() {
            write!(&mut result, "{}", self.serialize(value)).unwrap();
        }
        write!(&mut result, "e").unwrap();

        result
    }

    fn serialize_int(&self, int: i64) -> String {
        let mut result = String::new();
        write!(&mut result, "i{}e", int).unwrap();
        result
    }

    fn serialize_string(&self, string: &str) -> String {
        let mut result = String::new();
        write!(&mut result, "{}:{}", string.len(), string).unwrap();
        result
    }
}
