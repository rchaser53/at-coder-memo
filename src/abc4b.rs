#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use petgraph::unionfind::UnionFind;
use std::collections::{HashMap, HashSet, VecDeque};

#[fastout]
fn main() {
  input!{
    vals: [[char;4];4]
  }
  
  let mut result = vec![vec!['.';4];4];
  for i in 0..4 {
    for j in 0..4 {
      result[3-i][3-j] = vals[i][j];
    }
  }
  for i in 0..4 {
    let a = result[i].clone();
    println!("{}", a
      .into_iter()
      .map(|v| v.to_string())
      .collect::<Vec<String>>()
      .join(" ")
    );
  }
}