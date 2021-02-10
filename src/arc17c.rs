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

fn culc(
  vals: &Vec<usize>,
  n: usize
) -> HashMap<usize, usize> {
  let limit = 1 << n;
  let mut map = HashMap::new();
  for i in 0..limit {
    let mut temp = 0;
    for ii in 0..n {
      if i >> ii & 1 == 1 {
        temp += vals[ii];
      }
    }
    *map.entry(temp).or_insert(0) += 1;
  }
  map
}

fn main() {
  input! {
    n: usize,
    x: usize,
    vals: [usize;n]
  }
  
  if n < 17 {
    let map = culc(&vals, n);
    if let Some(v) = map.get(&x) {
      println!("{}", v);
    } else {
      println!("0");
    }
  } else {
    let map_a = culc(&vals, 16);
    let valsb = vals.clone()[16..].to_vec();
    let map_b = culc(&valsb, n - 16);
    
    let mut result = 0;
    for (av, an) in map_a {
      if x < av { continue }
      let diff = x - av;
      if let Some(bn) = map_b.get(&diff) {
        result += an * bn;
      }
    }
    println!("{}", result);
  }
}
