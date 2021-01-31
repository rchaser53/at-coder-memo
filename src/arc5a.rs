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
    vals: [String;n]
  }
  
  let set = hashset!{
    String::from("TAKAHASHIKUN"),
    String::from("Takahashikun"),
    String::from("takahashikun")
  };
  
  let mut count = 0;
  for i in 0..n-1 {
    if set.contains(&vals[i]) {
      count += 1;
    }
  }
  let len = vals[n-1].len();
  if set.contains(&vals[n-1][..len-1]) {
    count += 1;
  }
  println!("{}", count);
}