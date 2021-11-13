#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use std::cmp::Ordering;
use petgraph::unionfind::UnionFind;

fn main() {
  input!{
    n: usize,
    vals: [(String,usize);n]
  }
  
  let mut count = 0;
  for i in 0..n {
    count += vals[i].1;
  }
  
  for i in 0..n {
    if count / 2 + 1 <= vals[i].1 {
      println!("{}", vals[i].0);
      return
    }
  }
  println!("atcoder");
}
