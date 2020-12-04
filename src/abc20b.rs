#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use std::cmp::Ordering;
use petgraph::unionfind::UnionFind;

const MOD:usize = 1_000_000_007;

fn main() {
  input!{
    a: usize,
    b: usize,
  }
  
  let b_str = b.to_string();
  let b_len = b_str.len() as u32;
  println!("{}",
    (
      a * 10usize.pow(b_len) + b
    ) * 2
  );
}
