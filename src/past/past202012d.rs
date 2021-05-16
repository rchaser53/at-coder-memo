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
  let mut memo = vals.into_iter()
             .map(|v| {
               (v.clone(), v.trim_start_matches('0').to_string())
              })
             .collect::<Vec<(String,String)>>();
  memo.sort_by(|a, b| {
    if a.1 == b.1 {
      b.0.len().cmp(&a.0.len())
    } else if a.1.len() == b.1.len() {
      a.1.cmp(&b.1)
    } else {
      a.1.len().cmp(&b.1.len())
    }
  });
  
  for v in memo {
    println!("{}", v.0);
  }
}