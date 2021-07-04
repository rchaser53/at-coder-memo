// n = 10^(10^8)くらいまでのサイズが計算できるやつを考えてみた
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
use std::cmp::*;

const MOD:usize = 1_000_000_007;

fn main() {
  input!{
    n: usize,
    m: usize,
  }
  
  let mut memo = vec![String::from("");n+1];
  let mut left = 1;
  for i in 0..=n {
    let v = left / m;
    memo[i] = v.to_string();
    left = left % m;
    left *= 10;
  }
  
  println!("{}", memo
    .into_iter()
    .collect::<String>()
    .trim_start_matches('0')
  );
}