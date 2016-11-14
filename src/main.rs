extern crate rustorr;

fn main() {
    let s = "d9:publisheri3e:17:publisher-webpage15:www.example.com18:publisher.location4:homee";
    let iter = s.bytes();
    let mut hi = rustorr::bencode::bdecoder::Bdecoder::new(iter);
    println!("{:?}", hi.decode());
}
