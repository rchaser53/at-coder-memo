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
use num_complex::Complex;
 
const MOD:usize = 998244353;
const MAX:usize = 400010;

fn main() {
  input!{
    n:usize,
    mut vals:[usize;3]
  }
  vals.sort();

  let mut map = HashMap::new();
  for i in 0..10000 {
    map.insert(vals[2]*i, i);
  }
  
  let mut min = 10000;
  for i in 0..10000 {
    for j in 0..10000 {
      if 10000 < i + j { break }
      let v1 = i * vals[0];
      let v2 = j * vals[1];
      if n < v1 + v2 { break }
      let diff = n - v1 - v2;
      if let Some(k) = map.get(&diff) {
        min = std::cmp::min(i+j+k , min);
      }
    }
  }
  println!("{}",  min);
}

fn sol2() {
  input!{
    n:usize,
    mut vals:[usize;3]
  }
  vals.sort();

  let mut dp = vec![1_000_000;n+1];
  dp[0] = 0;
  for i in 0..=n {
    for j in 0..3 {
      let v = vals[j];
      if i < v { continue }
      dp[i] = std::cmp::min(dp[i], dp[i-v]+1);
    }
  }
  println!("{}", dp[n]);
}