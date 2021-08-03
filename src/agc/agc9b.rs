/* OUTPUT FILE */
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_assignments)]
use proconio::input;
use proconio::marker::*;
use std::collections::*;

fn dfs(
  g: &Vec<Vec<usize>>,
  ci: usize,
) -> usize {
  let mut memo = vec![];
  for &ni in &g[ci] {
    memo.push(dfs(g, ni));
  }
  memo.sort();
  let mut result = 0;
  for v in memo {
    result = std::cmp::max(result, v) + 1;
  }
  result
}

pub fn main(
) {
  input! {
    n:usize,
    vals:[Usize1;n-1]
  }

  let mut g = vec![vec![];n];
  for i in 0..n-1 {
    g[vals[i]].push(i+1);
  }
  println!("{}", dfs(&g, 0));
}