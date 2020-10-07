#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::{HashMap, HashSet, VecDeque};

#[fastout]
fn main() {
  input!{
    n: usize,
    m: usize,
    vals: [(Usize1,Usize1);m]
  }
  
  let mut memo: Vec<bool> = vec![false;n];
  let dist = n - 1;
  for (from, to) in vals.iter() {
    if *to == dist {
      memo[*from] = true;
    }
  }
  
  for (from, to) in vals {
    if from == 0 && memo[to] {
      println!("POSSIBLE");
      return
    }
  }
  println!("IMPOSSIBLE");
}