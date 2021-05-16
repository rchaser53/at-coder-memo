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
    vals: [usize;m]
  }
  
  let mut map = hashmap!{};
  for i in 1..=n {
    map.insert(i, i);
  }
  
  let mut now = 0;
  for cd_num in vals {
    if cd_num == now { continue }
    let case_index = map.remove(&cd_num).unwrap();
    map.insert(now, case_index);
    now = cd_num;
  }
  
  let mut result = map
    .into_iter()
    .collect::<Vec<(usize, usize)>>();
  result.sort_by(|a,b| a.1.cmp(&b.1));
  for (v, _) in result {
    println!("{}", v);  
  }
}