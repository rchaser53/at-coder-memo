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
    n: usize,
  }
  
  let limit = std::cmp::min(9*18, n);
  let mut result = vec![];
  let mut i = 0;
  while i <= limit {
    let v = n - i;
    let sum = v + v
      .to_string()
      .chars()
      .into_iter()
      .map(|v| v.to_string().parse::<usize>().unwrap())
      .sum::<usize>();
    if sum == n {
      result.push(v);
    }
    i += 1;
  }
  
  result.sort();
  println!("{}", result.len());
  for v in result {
    println!("{}", v);
  }
}