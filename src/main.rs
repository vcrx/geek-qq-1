use md5;
use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut f = File::open(env::current_exe().unwrap()).unwrap();
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer).unwrap();
    let digest = md5::compute(&buffer);
    println!("{:x}", digest);
}
