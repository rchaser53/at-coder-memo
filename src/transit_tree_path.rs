#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::{HashMap, HashSet, VecDeque};

fn dfs(
  routes: &Vec<Vec<(usize, usize)>>,
  memo: &mut Vec<usize>,
  dist: usize,
  last: usize,
  next: usize,
  v: usize,
) {
  for (to, val) in routes[next].iter() {
    let to = *to;
    if memo[to] != 0 { continue }
    if to != last {
      memo[to] = val + v;
      dfs(routes, memo, dist, next, to, val + v);
    }
  }
}

#[fastout]
fn main() {
  input!{
    n: usize,
    vals: [(Usize1, Usize1, usize);n-1],
    q: usize,
    k: Usize1,
    queries: [(Usize1,Usize1);q]
  }
  
  let mut memo: Vec<usize> = vec![0;n];
  let mut routes: Vec<Vec<(usize,usize)>> = vec![vec![];n];
  for i in 0..n-1 {
    let (from, to, val) = vals[i].clone();
    routes[from].push((to, val));
    routes[to].push((from, val));
  }
  
  for i in 0..n {
    if i == k || memo[i] != 0 { continue }
    dfs(&routes, &mut memo, i, 10_000_000, k, 0);
  }
  
  for (from, to) in queries {
    println!("{}", memo[from] + memo[to]);
  }
}