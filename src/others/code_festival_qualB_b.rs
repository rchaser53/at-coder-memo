#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::{HashMap, HashSet, VecDeque};

#[fastout]
fn main() {
  input!{
    n: usize,
    k: usize,
    vals: [usize;n]
  }
  
  let mut total = 0;
  for i in 0..n {
    total += vals[i];
    if k <= total {
      println!("{}", i + 1);
      return
    }
  }
}
