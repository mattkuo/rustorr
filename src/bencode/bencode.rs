use std::collections::BTreeMap;

#[derive(Debug)]
pub enum Bencode {
    Dict(BTreeMap<String, Bencode>),
    List(Vec<Bencode>),
    Int(i64),
    Str(String),
}