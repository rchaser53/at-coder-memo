#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use std::cmp::Ordering;
use petgraph::unionfind::UnionFind;

fn main() {
  input!{
    n: usize,
    q: usize,
    vals: [(Usize1, Usize1);q]
  }
  
  let mut memo = vec![(0, 0);n];
  for (l, r) in vals {
    memo[l].0 += 1;
    if r < n-1 {
      memo[r+1].1 += 1;
    }
  }
  
  let mut result = vec![0;n];
  result[0] = memo[0].0;
  for i in 1..n {
    let (l, r) = memo[i];
    result[i] += result[i-1] + l - r;
  }
  
  println!("{}", result
    .into_iter()
    .map(|v| {
      if v % 2 == 0 {
        String::from("0")
      } else {
        String::from("1") 
      }
    })
    .into_iter()
    .collect::<String>()
  );
}
