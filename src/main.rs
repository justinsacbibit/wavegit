extern crate flate2;
extern crate rustc_serialize;

use std::io::Read;
use std::fs::File;

use flate2::read::ZlibDecoder;
use rustc_serialize::hex::ToHex;

// Returns Vector of tuples that hold (mode, filename, hash)
fn split_tree_content(tree_content: &[u8]) -> Vec<(Vec<u8>, Vec<u8>, Vec<u8>)> {
    let mut result: Vec<(Vec<u8>, Vec<u8>, Vec<u8>)> = Vec::new();
    let mut it = tree_content.iter();
    loop {
        let mut mode: Vec<u8> = Vec::new();
        let mut filename: Vec<u8> = Vec::new();
        let mut hash: Vec<u8> = Vec::new();
        loop {
            let byte = match it.next() {
                Some(x) => *x,
                None => { return result; }
            };
            if byte == 0x20 {
                break;
            }
            mode.push(byte);
        }
        loop {
            let byte = *it.next().unwrap();
            if byte == 0x0 {
                break;
            }
            filename.push(byte);
        }
        for _ in 0..20 {
            let byte = *it.next().unwrap();
            hash.push(byte);
        }
        result.push((mode, filename, hash));
    }
}

fn main() {
    let cargo_toml_hash = "2ba5f3f32c8b887b7e6cee0dcd8823e04987f7fc";
}
