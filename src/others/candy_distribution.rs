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

#[fastout]
fn main() {
  input!{
    n: usize,
    m: usize,
    vals: [usize;n]
  }
  
  let mut temp = 0;
  let mut map = HashMap::new();
  map.insert(0, 1u128);
  for i in 0..n {
    temp += vals[i];
    temp %= m;
    *map.entry(temp).or_insert(0) += 1;
  }
  
  let mut result = 0;
  for (_, v) in map {
    result += v * (v - 1) / 2;
  }
  println!("{}", result);
}