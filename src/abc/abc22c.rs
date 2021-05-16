#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use std::cmp::Ordering;
use petgraph::unionfind::UnionFind;
use petgraph::algo::dijkstra;
use petgraph::graph::{NodeIndex, UnGraph};

const MOD:usize = 1_000_000_007;

fn main() {
  input!{
    n: usize,
    m: usize,
    vals: [(Usize1, Usize1, usize);m]
  }
  
  let mut to_one = HashMap::new();
  let mut memo = vec![vec![1_000_000_000;n];n];
  for i in 0..m {
    let (l, r, v) = vals[i];
    memo[l][r] = v;
    memo[r][l] = v;
    
    if l == 0 {
      to_one.insert(r, v);
    } else if r == 0 {
      to_one.insert(l, v);
    }
  }
  
  for i in 0..n {
    memo[i][i] = 0;
  }
  
  for i in 1..n {
    for j in 1..n {
      for k in 1..n {
        memo[j][k] = std::cmp::min(
          memo[j][k], memo[j][i] + memo[i][k]
        );
      }
    }
  }
  
  let mut min = 1_000_000_000;
  for i in 1..n {
    if let Some(v) = to_one.get(&i) {
      let mut s_min = 1_000_000_000;
      for j in 1..n {
        if i == j || memo[i][j] == 1_000_000_000 { continue }
        if let Some(vv) = to_one.get(&j) {
          s_min = std::cmp::min(s_min, memo[i][j] + v + vv);
        }
      }
      if s_min != 1_000_000_000 {
        min = std::cmp::min(min, s_min);
      }
    }
  }
  
  if min == 1_000_000_000 {
    println!("-1");
  } else {
    println!("{}", min);  
  }
}