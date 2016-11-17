// highlight.rs
#![feature(rustdoc)]

extern crate rustdoc;

use std::io::{self, Read};
use rustdoc::html::highlight::render_with_highlighting;

fn main() {
    let mut buf = String::new();
    let stdin = io::stdin();
    stdin.lock().read_to_string(&mut buf).unwrap();
    let output = render_with_highlighting(&buf, None, None, None);
    print!("{}", output);
}
