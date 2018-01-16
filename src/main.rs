extern crate getopts;
extern crate rustorr;

use getopts::Options;
use std::fs::File;
use std::env;
use std::io::Read;

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
    
    let mut decoder = rustorr::bencode::bdecoder::Bdecoder::new(file_buffer.iter().cloned());
    println!("{:?}", decoder.decode());
}
