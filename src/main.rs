extern crate rustorr;

fn main() {
    let s = "6:hello";
    let iter = s.bytes();
    let mut hi = rustorr::bencode::Bdecoder::new(iter);
    hi.decode();
}
