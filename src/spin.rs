#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use std::cmp::Ordering;
use petgraph::unionfind::UnionFind;

fn main() {
  input!{
    n: usize,
    vals: [Chars;n]
  }
  
  let mut result = vec![vec!['a';n];n];
  for i in 0..n {
    for ii in 0..n {
      result[i][ii] = vals[n-1-ii][i];
    }
  }
  
  for row in result {
    println!("{}", row
      .into_iter()
      .map(|v| v.to_string())
      .collect::<String>()
    );
  }
}