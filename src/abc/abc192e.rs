#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use itertools::Itertools;
use maplit::{btreemap, btreeset, hashmap, hashset};
use petgraph::unionfind::UnionFind;
// use petgraph::algo::dijkstra;
use petgraph::graph::{NodeIndex, DiGraph, UnGraph};
use permutohedron::{Heap, heap_recursive};
use std::collections::*;
use superslice::Ext;

const MOD:usize = 1_000_000_007;

fn adjacency_list(
  n: usize,
  uvw: &Vec<(usize, usize, usize, usize)>
) -> Vec<Vec<(usize, usize, usize)>> {
  let mut e = vec![vec![]; n];
  for &(u, v, w, k) in uvw.iter() {
    e[u].push((v, w, k));
    e[v].push((u, w, k));
  }
  e
}

fn dijkstra(
  n: usize,
  default_val: usize,
  graph: &Vec<Vec<(usize, usize, usize)>>,
  start: usize,
) -> Vec<usize> {
  let mut score = vec![default_val;n];
  let mut pq = BinaryHeap::new();
  score[start] = 0;
  pq.push(std::cmp::Reverse((0, start)));
  while let Some(std::cmp::Reverse((from_w, from))) = pq.pop() {
    if score[from] < from_w {
      continue
    }
    
    // to, value, k
    for &(to, to_w, k) in graph[from].iter() {
      let mut from_w = from_w;
      let re = from_w % k;
      if re != 0 {
        from_w += k - re;
      }
      
      let w = from_w + to_w;
      if w < score[to] {
        score[to] = w;
        pq.push(std::cmp::Reverse((w, to)));
      }
    }
  }
  score
}

#[fastout]
fn main() {
  input!{
    n: usize,
    m: usize,
    x: Usize1,
    y: Usize1,
    vals: [(Usize1, Usize1, usize, usize);m]
  }
  
  let inf = 1_000_000_000_000_000_000;
  let mut ad_list = adjacency_list(n, &vals);
  let result = dijkstra(n, inf, &ad_list, x);
  if result[y] != inf {
    println!("{}", result[y]);
  } else {
    println!("-1");
  }
}