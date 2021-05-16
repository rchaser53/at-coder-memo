#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use itertools::Itertools;
use maplit::{btreemap, btreeset, hashmap, hashset};
use petgraph::unionfind::UnionFind;
use petgraph::algo::dijkstra;
use petgraph::graph::{NodeIndex, DiGraph, UnGraph};
use permutohedron::{Heap, heap_recursive};
use std::collections::*;
use superslice::*;
use std::cmp::Ordering;

const MOD:usize = 1_000_000_007;
const MAX: usize = 1000;

fn main() {
  input!{
    n:usize,
    mut vals:[(isize, usize);n]
  }
  
  vals.sort_by(|a, b| {
    let v = a.1.cmp(&b.1);
    if v == Ordering::Equal {
      a.0.cmp(&b.0)
    } else {
      v
    }
  });
  let mut last = vals[0].1;
  let mut memo = vec![vec![]];
  for i in 0..n {
    let last_i = memo.len()-1;
    if vals[i].1 == last {
      memo[last_i].push(vals[i].0);
    } else {
      memo.push(vec![vals[i].0]);
      last = vals[i].1;
    }
  }
  
  let len = memo.len();
  let inf = 1_000_000_000_000_000;
  let mut dp = vec![(inf, inf);len+1];
  dp[0] = (0,0);
  
  let mut last_f = 0;
  let mut last_l = 0;
  for i in 0..len {
    let c_f = memo[i][0];
    let c_l = memo[i][memo[i].len()-1];

    let f_f = (last_f - c_l).abs() + (c_l - c_f).abs();
    let f_l = (last_f - c_f).abs() + (c_l - c_f).abs();
    let l_f = (last_l - c_l).abs() + (c_l - c_f).abs();
    let l_l = (last_l - c_f).abs() + (c_l - c_f).abs();
    
    dp[i+1].0 = std::cmp::min(dp[i+1].0, dp[i].0 + f_f);
    dp[i+1].0 = std::cmp::min(dp[i+1].0, dp[i].1 + l_f);
    dp[i+1].1 = std::cmp::min(dp[i+1].1, dp[i].0 + f_l);
    dp[i+1].1 = std::cmp::min(dp[i+1].1, dp[i].1 + l_l);
    last_f = c_f;
    last_l = c_l;
  }
    
  println!("{}", std::cmp::min(
    dp[len].0 + last_f.abs(),
    dp[len].1 + last_l.abs()
  ));
}