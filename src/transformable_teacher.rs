#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use petgraph::unionfind::UnionFind;

#[fastout]
fn main() {
  input!{
    n: usize,
    m: usize,
    mut cs: [isize;n],
    mut ts: [isize;m]
  }
  
  cs.sort();
  let mut l_memo = vec![0;n+1];
  for (i, &h) in cs.iter().enumerate() {
    l_memo[i+1] = l_memo[i];
    if i % 2 == 0 {
      l_memo[i+1] -= h;
    } else {
      l_memo[i+1] += h;
    }
  }
  
  let mut r_memo = vec![0;n+1];
  for (i, &h) in cs.iter().rev().enumerate() {
    r_memo[i+1] = r_memo[i];
    if i % 2 == 0 {
      r_memo[i+1] += h;
    } else {
      r_memo[i+1] -= h;
    }
  }
  
  let mut ans = isize::max_value();
  for v in ts {
    let i = cs
      .binary_search_by_key(&(v, 1), |vv| (*vv, 0))
      .unwrap_err();
    let val = if i % 2 == 1 {
        l_memo[i] + v + r_memo[n-i]
      } else {
        l_memo[i] - v + r_memo[n-i]
      };
      ans = std::cmp::min(ans, val);
  }
  println!("{}", ans);
}