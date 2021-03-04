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

const MOD:usize = 1_000_000_007;

#[fastout]
fn main() {
  input!{
    n: usize,
    vals: [(usize,Usize1);n]
  }
  
  let mut memo = vec![(0,0,0);n];
  for i in 0..n {
    let (rate, pattern) = vals[i];
    memo[i] = (i, rate, pattern);
  }
  memo.sort_by(|a,b| a.1.cmp(&b.1));

  let mut map = HashMap::new();
  for i in 0..n {
    let (ii, rate, p) = memo[i];
    let mut entry = map.entry(rate).or_insert((0, vec![]));
    entry.0 = i;
    entry.1.push((ii, p));
  }
  let mut result = vec![(0,0,0);n];
  for (_, (last_index, ps)) in map {
    let mut pattern = vec![0;3];
    let ps_num = ps.len();
    for i in 0..ps_num {
      pattern[ps[i].1] += 1;
    }
    
    let base_win = last_index - ps_num + 1;
    let base_lose = n - 1 - last_index;
    for (ri, p) in ps {
      result[ri].0 = base_win + pattern[(p+1)%3];
      result[ri].1 = pattern[p] - 1;
      result[ri].2 = base_lose + pattern[(p+2)%3];
    }
  }
  
  for (win, draw, lose) in result {
    println!("{} {} {}", win, lose, draw);
  }
}