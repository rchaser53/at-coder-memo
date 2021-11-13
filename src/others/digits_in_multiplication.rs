#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::{HashMap, HashSet, VecDeque};

#[fastout]
fn main() {
  input!{
    n: usize,
  }
  
  let mut min = n.to_string().len();
  let mut i = 2;
  while i * i <= n {
    if n % i == 0 {
      let v = std::cmp::max((n / i).to_string().len(), i.to_string().len());
      min = std::cmp::min(v, min);
    }
    i += 1;
  }
  println!("{}", min);
}