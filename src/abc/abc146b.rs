/* OUTPUT FILE */
#![allow(unused_imports)]
use permutohedron::heap_recursive;
use proconio::input;
use proconio::marker::*;
use std::collections::*;

pub fn main(
) {
    input! {
      n:usize,
      mut s:Chars
    }

    let dict = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let dict = dict.chars().into_iter().collect::<Vec<char>>();

    for i in 0..s.len() {
      let ci = (s[i] as u8 - 'A' as u8) as usize;
      s[i] = dict[(ci + n) % dict.len()];
    }
    println!("{}", s.iter().map(|v| v.to_string()).collect::<String>());
}
