extern crate rustorr;

fn main() {
    let s = "d3:cow3:moo4:spam4:eggse";
    let iter = s.bytes();
    let mut hi = rustorr::bencode::bdecoder::Bdecoder::new(iter);
    println!("{:?}", hi.decode());
}
