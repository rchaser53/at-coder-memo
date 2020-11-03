#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use petgraph::unionfind::UnionFind;

#[fastout]
fn main() {
  input!{
    n: usize,
    k: usize,
    s: usize,
  }
  
  let mut v = 1_000_000_000;
  if s == 1_000_000_000 {
    v -= 1;
  }

  let mut result = vec![v;n];
  for i in 0..k {
    result[i] = s;
  }
  
  println!("{}", result
    .into_iter()
    .map(|v| v.to_string())
    .collect::<Vec<String>>()
    .join(" ")
  );
}