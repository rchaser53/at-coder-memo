#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use std::cmp::Ordering;
use maplit::{btreemap, btreeset, hashmap, hashset};
use petgraph::unionfind::UnionFind;
use petgraph::algo::dijkstra;
use petgraph::graph::{NodeIndex, UnGraph};
use num::Num;
use std::f64::consts::PI;

const MOD:usize = 1_000_000_007;

fn main() {
  input!{
    n:usize,
    a:usize,
    vals:[usize;n]
  }
  
  let limit = n*a+1;
  let mut dp = vec![vec![0;limit]];
  dp[0][0] = 1usize;
  for i in 0..n {
    let mut new_vec = dp.clone();
    new_vec.push(vec![0;limit]);
    let v = vals[i];
    for ii in 1..=i+1 {
      for iii in 0..limit {
        if iii < v { continue }
        new_vec[ii][iii] += dp[ii-1][iii-v];
      }
    }
    dp = new_vec;
  }

  let mut count = 0usize;
  for i in 1..=n {
    count += dp[i][a*i];
  }
  println!("{}", count);
}