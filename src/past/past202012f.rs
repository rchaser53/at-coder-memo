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
    vals: [(Usize1, Usize1, Usize1);m]
  }
  
  let mut sets = HashSet::new();
  for i in 0..m {
    let (a, b, c) = vals[i];
    sets.insert((1 << a) + (1 << b) + (1 << c));
  }
  
  let mut result = 0;
  let limit = 1 << n;
  for i in 0..limit {
    let mut out = false;
    for v in sets.iter() {
      if sets.contains(&(i & v)) {
        out = true;
        break
      }
    }
    if out { continue }
    
    let mut count = 0;
    for ii in 0..n {
      if i >> ii & 1 == 1 { continue }
      let new = (1 << ii) | i;
      
      let mut out = false;
      for v in sets.iter() {
        if sets.contains(&(new & v)) {
          out = true;
          break
        }
      }
      if out {
        count += 1;
      }
    }
    result = std::cmp::max(result, count);
  }
  println!("{}", result);
}