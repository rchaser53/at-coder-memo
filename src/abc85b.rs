#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;
use std::collections::VecDeque;

fn main() {
  input!{
    n: usize,
    mut vals: [usize;n]
  }
  
  vals.sort();
  vals.dedup();
  println!("{}", vals.len());
}