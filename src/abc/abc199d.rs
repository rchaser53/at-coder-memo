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
use superslice::*;

const MOD:usize = 1_000_000_007;
const MAX:usize = 1000;

fn culc(
  parents:&mut Vec<isize>,
  neighbors:&Vec<Vec<usize>>,
  v:usize,
  p:usize
) {
  parents[v] = p as isize;
  for &u in neighbors[v].iter() {
    if parents[u] == -1 {
      culc(parents, neighbors, u, v);
    }
  }
}

fn dfs(
  l:usize,
  colors:&mut Vec<isize>,
  parents:&mut Vec<isize>,
  neighbors:&Vec<Vec<usize>>,
) -> usize {
  (0..3).map(|s| {
    if neighbors[l].iter().any(|&r| colors[r] == s) {
      return 0;
    }
    colors[l] = s;

    let mut count = 1;
    neighbors[l].iter().for_each(|&r| {
      if parents[r] == l as isize {
        count *= dfs(r, colors, parents, neighbors);
      }
    });
    colors[l] = -1;
    count
  }).sum()
}

fn main() {
  input!{
    n:usize,
    m:usize,
    vals:[(Usize1,Usize1);m]
  }
  
  let mut neighbors = vec![vec![];n];
  for (from, to) in vals {
    neighbors[from].push(to);
    neighbors[to].push(from);
  }
  
  let default = 1_000_000;
  let mut colors = vec![-1;n];
  let mut parents = vec![-1;n];
  for i in 0..n {
    if parents[i] == -1 {
      culc(&mut parents, &neighbors, i, default);
    }
  }
    
  let mut result = 1;
  for l in 0..n {
    if parents[l] == default as isize {
      result *= dfs(l, &mut colors, &mut parents, &neighbors);
    }
  }
  println!("{}", result);
}