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
    vals: [(Usize1, Usize1, isize);m]
  }
  let mut neighbors = vec![vec![];n];
  for (l, r, d) in vals {
    neighbors[l].push((r, d));
    neighbors[r].push((l, -d));
  }
  
  let mut memo = vec![None;n];
  let mut seen = vec![false;n];
  for i in 0..n {
    if !dfs(i, &neighbors, &mut memo, &mut seen) {
      println!("No");
      return
    }
  }
  println!("Yes");
}

fn dfs(
  i: usize,
  neighbors: &Vec<Vec<(usize, isize)>>,
  memo: &mut Vec<Option<isize>>,
  seen: &mut Vec<bool>
) -> bool {
  if seen[i] {
    return true
  }
  if memo[i].is_none() {
    memo[i] = Some(0);
  }
  
  let x = memo[i].unwrap();
  for &(w, d) in &neighbors[i] {
    if let Some(y) = memo[w] {
      if x + d != y {
        return false;
      }
    } else {
      memo[w] = Some(x+d);
      if !dfs(w, neighbors, memo, seen) {
        return false;
      }
    }
  }
  true
}
