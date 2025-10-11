/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::marker::PhantomData;
use std::cmp::*;
use std::collections::*;

fn dfs(memo: &Vec<Vec<usize>>, seen: &mut Vec<bool>, u:usize) {
  seen[u] = true;
  for &v in &memo[u] {
    if !seen[v] {
      dfs(memo, seen, v);
    }
  }
}

fn main() {
  input! {
    n:usize,
    ab:[(usize,usize);n],
  }

  let mut memo = vec![vec![];n+1];
  for i in 1..=n {
    memo[ab[i-1].0].push(i);
    memo[ab[i-1].1].push(i);
  }
  let mut seen = vec![false;n+1];
  dfs(&memo, &mut seen, 0);
  let count = seen.iter().filter(|&&x| x).count();
  println!("{}", count - 1);
}