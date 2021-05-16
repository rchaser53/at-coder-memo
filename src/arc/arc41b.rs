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
    m: usize,
    mut base: [Chars;n]
  }
  let mut rows = vec![vec![0;m];n];
  for i in 0..n {
    rows[i] = base[i]
      .clone()
      .into_iter()
      .map(|v| v.to_string().parse::<usize>().unwrap())
      .collect();
  }
  
  let mut result = vec![vec![0;m];n];
  for r in 1..n-1 {
    for c in 1..m-1 {
      let north = rows[r-1][c];
      let west = rows[r][c-1];
      let east = rows[r][c+1];
      let south = rows[r+1][c];
      
      let mut v = std::cmp::min(north, west);
      v = std::cmp::min(v, east);
      v = std::cmp::min(v, south);
      if v != 0 {
        result[r][c] = v;
        rows[r-1][c] -= v;
        rows[r+1][c] -= v;
        rows[r][c-1] -= v;
        rows[r][c+1] -= v;
      }
    }
  }
  for row in result {
    println!("{}", row.into_iter().map(|v| v.to_string()).collect::<String>());
  }
}