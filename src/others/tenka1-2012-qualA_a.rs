#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use std::cmp::Ordering;
use maplit::{btreemap, btreeset, hashmap, hashset};
use petgraph::unionfind::UnionFind;
use petgraph::algo::dijkstra;
use petgraph::graph::{NodeIndex, UnGraph};

const MOD:usize = 1_000_000_007;

fn main() {
  input!{
    n: usize,
  }
  
  let mut memo = vec![0,0,1];
  for i in 1..n {
    memo[2] += memo[1];
    memo[1] = memo[0];
    memo[0] = memo[2];
  }
  println!("{}", memo.iter().sum::<usize>());
}
