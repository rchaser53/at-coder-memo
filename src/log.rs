#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use std::cmp::Ordering;
use petgraph::unionfind::UnionFind;

const MOD:usize = 1_000_000_007;

fn main() {
  input!{
    n: usize,
  }
  
  let mut i = 1;
  let mut total = 0;
  loop {
    total += i;
    if n+1 < total { break }
    i += 1;
  }
  println!("{}", n - (i - 1) + 1);
}
