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

const MOD:usize = 998244353;
const MAX:usize = 200_000;

#[fastout]
fn main() {
  input!{
    h:usize,
    w:usize,
    vals:[[usize;w];h],
  }
  
  let mut rows = vec![0;h];
  let mut columns = vec![0;w];
  for i in 0..h {
    for ii in 0..w {
      rows[i] += vals[i][ii];
      columns[ii] += vals[i][ii];
    }
  }

  let mut result = vec![vec![0;w];h];
  for i in 0..h {
    for ii in 0..w {
      result[i][ii] = rows[i] + columns[ii] - vals[i][ii];
    }
    println!("{}", result[i]
      .iter()
      .map(|v| v.to_string())
      .join(" ")
    );
  }
}