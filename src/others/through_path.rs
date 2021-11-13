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

// 木、部分木更新、IMOS

fn dfs(
  points: &Vec<Vec<usize>>,
  depths: &mut Vec<isize>,
  next: usize,
  val: isize
) {
  depths[next] = val;
  for i in 0..points[next].len() {
    let ni = points[next][i];
    if depths[ni] == -1 {
      dfs(points, depths, ni, val+1);
    }
  }
}

// IMOS
fn culc_dfs(
  points: &Vec<Vec<usize>>,
  result: &mut Vec<isize>,
  next: usize,
  last: usize,
  val: isize,
) {
  result[next] += val;
  let nv = result[next];
  for i in 0..points[next].len() {
    let ni = points[next][i];
    if ni != last {
      culc_dfs(points, result, ni, next, nv);
    }
  }
}

fn main() {
  input!{
    n: usize,
    edges: [(Usize1, Usize1);n-1],
    q: usize,
    queries: [(usize, Usize1, isize);q]
  }
  
  let mut points = vec![vec![];n];
  let mut depths = vec![-1;n];
  let mut result = vec![0;n];
  for i in 0..n-1 {
    let (from, to) = edges[i];
    points[from].push(to);
    points[to].push(from);
  }
  
  dfs(&points, &mut depths, 0, 0);
  
  for (t, ti, v) in queries {
    let (a, b) = if t == 1 {
      (edges[ti].0, edges[ti].1)
    } else {
      (edges[ti].1, edges[ti].0)
    };
    
    // 親 -> 子
    if depths[a] < depths[b] {
      result[0] += v;
      result[b] -= v;
    }
    // 子 -> 親 
    else {
      result[a] += v;
    }
  }
  culc_dfs(&points, &mut result, 0, 1_000_000_000, 0);
  
  for v in result {
    println!("{}", v);
  }
}
