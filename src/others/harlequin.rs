#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::{HashMap, HashSet, VecDeque};

#[fastout]
fn main() {
  input!{
    n: usize,
    vals: [usize;n]
  }
  
  for v in vals {
    if v % 2 == 1 {
      println!("first");
      return
    }
  }
  println!("second");
}