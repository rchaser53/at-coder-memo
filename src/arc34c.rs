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
use superslice::Ext;

const MOD:usize = 1_000_000_007;

#[fastout]
fn main() {
  input!{
    a: usize,
    b: usize
  }
  
  let mut map = HashMap::new();
  for limit in b+1..=a {
    let mut i = 2;
    let mut v = limit; 
    while i * i <= limit {
      if v % i == 0 {
        *map.entry(i).or_insert(0) += 1;
        v /= i;
      } else {
        i += 1;
      }
    }
    if v != 1 {
      *map.entry(v).or_insert(0) += 1;
    }
  }

  let mut result = 1;
  for (_, v) in map {
    result *= (v+1);
    result %= MOD;
  }
  println!("{}", result);
}