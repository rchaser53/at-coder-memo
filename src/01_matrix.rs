#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use petgraph::unionfind::UnionFind;

fn main() {
  input!{
    h: usize,
    w: usize,
    a: usize,
    b: usize
  }
  
  let mut memo:Vec<Vec<String>> = vec![vec![String::from("");w];h];
  for i in 0..h {
    for ii in 0..w {
      if i < b && ii < a
      || b <= i && a <= ii {
        memo[i][ii] = String::from("0");
      } else {
        memo[i][ii] = String::from("1");
      }
    }
  }
  
  for row in memo {
    println!("{}", row.into_iter().collect::<String>());
  }
}