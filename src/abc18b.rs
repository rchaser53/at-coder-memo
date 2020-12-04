#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use std::cmp::Ordering;
use petgraph::unionfind::UnionFind;

const MOD:usize = 1_000_000_007;

fn main() {
  input!{
    s: Chars,
    n: usize,
    vals: [(Usize1,Usize1);n]
  }
  
  let mut memo = vec![0;s.len()];
  for i in 0..s.len() {
    memo[i] = i;
  }
  
  let mut temp = vec![0;s.len()];
  for (l, r) in vals {
    for i in l..=r {
      temp[r+l-i] = memo[i];
    }
    for i in l..=r {
      memo[i] = temp[i];
    }
  }
  
  println!("{}", memo
    .into_iter()
    .map(|i| s[i].to_string())
    .collect::<String>()
  );
}