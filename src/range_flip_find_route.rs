#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use petgraph::unionfind::UnionFind;

fn main() {
  input!{
    h: usize,
    w: usize,
    rows: [Chars;h]
  }
  
  let mut memo = vec![vec![0;w];h];
  if rows[0][0] == '#' {
    memo[0][0] = 1;
  }
  
  let mut last = rows[0][0];
  for i in 1..w {
    memo[0][i] = memo[0][i-1];
    if last != rows[0][i] {
      memo[0][i] += 1;
    }
    last = rows[0][i];
  }
  
  last = rows[0][0];
  for i in 1..h {
    memo[i][0] = memo[i-1][0];
    if last != rows[i][0] {
      memo[i][0] += 1;
    }
    last = rows[i][0];
  }
  
  for i in 1..h {
    for ii in 1..w {
      let mut top = memo[i-1][ii];
      if rows[i-1][ii] != rows[i][ii] {
        top += 1;
      }
      
      let mut left = memo[i][ii-1];
      if rows[i][ii-1] != rows[i][ii] {
        left += 1;
      }
      
      memo[i][ii] = std::cmp::min(left, top);
    }
  }
  println!("{}", memo[h-1][w-1] / 2 + memo[h-1][w-1] % 2);
}