extern crate getopts;
extern crate url;
extern crate rustorr;

use getopts::Options;
use url::Url;
use std::collections::BTreeMap;
use std::fs::File;
use std::env;
use std::io::Read;
use rustorr::bencode::deserializer::BencodeDeserializer;
use rustorr::bencode::bencode::Bencode;
use rustorr::bencode::serializer::BencodeSerializer;

fn main() {
    let args: Vec<String> = env::args().collect();
    // let program: String = args[0].clone();

    let opts: Options = Options::new();

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };

    let input: String = if !matches.free.is_empty() {
        matches.free[0].clone()
    } else {
        return;
    };

    let mut file_buffer = Vec::new();
    File::open(input).unwrap().read_to_end(&mut file_buffer).unwrap();
    
    let mut decoder = BencodeDeserializer::new(file_buffer.iter().cloned());

    let bencoded = decoder.deserialize();

    let dictionary = match bencoded {
        Bencode::Dict(map) => map,
        _ => panic!("Not a map!!")
    };

//    let a = dictionary.get("announce").unwrap();

    //println!("{:?}", search_string(&dictionary, "announce"));
    let a = dictionary.get("info").unwrap();

    let serializer = BencodeSerializer::new();

    println!("{:?}", serializer.serialize(a));
}

//fn tracker_request(announce_url: &String) {
//    let url = Url::parse_with_params(announce_url, &[])
//
//}

fn search_string(dictionary: &BTreeMap<String, Bencode>, search_term: &str) -> String {
    let retrieved: Option<&Bencode> = dictionary.get(search_term);

    if let Some(&Bencode::Str(ref string)) = retrieved {
        string.clone()
    } else {
        panic!("Not a string");
    }
}
