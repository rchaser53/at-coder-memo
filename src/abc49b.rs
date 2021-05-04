#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;

#[fastout]
fn main() {
  input!{
    h:usize,
    w:usize,
    rows:[Chars;h]
  }
  for i in 0..rows.len() {
    let v = rows[i].iter().map(|v| v.to_string()).collect::<String>();
    println!("{}", &v);
    println!("{}", &v);
  }
}