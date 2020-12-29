#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use std::cmp::*;
use maplit::{btreemap, btreeset, hashmap, hashset};
use petgraph::unionfind::UnionFind;
use petgraph::algo::dijkstra;
use petgraph::graph::{NodeIndex, DiGraph, UnGraph};
use permutohedron::{Heap, heap_recursive};

const MOD:usize = 1_000_000_007;

fn main() {
  input!{
    s: Chars
  }
   
  let mut map = HashMap::new();
  for c in s {
    *map.entry(c).or_insert(0) += 1;
  }
  
  let mut max = 0;
  let mut result = 'a';
  for (key, v) in map {
    if max < v {
      result = key;
      max = v;
    }
  }
  println!("{}", result);
}