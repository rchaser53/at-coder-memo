#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use std::cmp::Ordering;
use petgraph::unionfind::UnionFind;

fn main() {
  input!{
    s: usize,
    p: usize,
  }
  
  let mut n = 1;
  while n <= p / n + p % n {
    if p % n == 0 {
      let m = p / n;
      if n + m == s {
        println!("Yes");
        return
      }
    }
    n += 1;
  }
  println!("No");
}
