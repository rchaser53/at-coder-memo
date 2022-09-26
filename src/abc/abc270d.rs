/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::Ordering;

fn dfs(memo: &mut Vec<Option<usize>>, a: &Vec<usize>, cv: usize) -> usize {
  if let Some(v) = memo[cv] {
    return v
  }

  let mut max = 0;
  for &v in a {
    if cv < v {
      continue
    }
    max = max.max(cv-dfs(memo,a,cv-v));
  }
  memo[cv] = Some(max);
  max
}

fn main() { 
  input! { 
    n:usize,
    k:usize,
    a:[usize;k]
  }

  let mut memo = vec![None;n+1];
  memo[0] = Some(0);
  for &v in &a {
    memo[v] = Some(v);
  }
  dfs(&mut memo, &a, n);
  println!("{}", memo[n].unwrap());
}