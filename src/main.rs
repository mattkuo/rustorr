extern crate rustorr;

fn main() {
    let s = "d4:spaml1:a1:bee";
    let iter = s.bytes();
    let mut hi = rustorr::bencode::bdecoder::Bdecoder::new(iter);
    println!("{:?}", hi.decode());
}
