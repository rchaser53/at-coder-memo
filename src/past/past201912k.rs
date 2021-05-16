#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use std::cmp::*;
use maplit::{btreemap, btreeset, hashmap, hashset};
use petgraph::unionfind::UnionFind;
use petgraph::algo::dijkstra;
use petgraph::graph::{NodeIndex, DiGraph, UnGraph};
use permutohedron::{Heap, heap_recursive};

const MOD:usize = 1_000_000_007;

fn main() {
  input!{
    n: usize,
    def: [isize;n],
    q: usize,
    queries: [(usize, usize);q]
  }
   
  let mut base = 0;
  let mut memo = vec![vec![];n+1];
  for (i, v) in def.into_iter().enumerate() {
    if v == -1 {
      base = i+1;
    } else {
      memo[v as usize].push(i+1);
    }
  }
  
  let mut pos = vec![1_000_000_000;n+1];
  let mut froms = vec![1_000_000_000;n+1];
  let mut tos = vec![1_000_000_000;n+1];
  let mut deque = VecDeque::new();
  deque.push_back(base);
  let mut i = 0;
  while let Some(current) = deque.pop_back() {
    if froms[current] == 1_000_000_000 {
      froms[current] = i;
    } else {
      tos[current] = i;
      continue
    }
    
    pos[current] = i;
    deque.push_back(current);
    for v in memo[current].iter() {
      deque.push_back(*v as usize);
    }
    i += 1;
  }
  
  for (a, b) in queries {
    if froms[b] < pos[a] && pos[a] < tos[b] {
      println!("Yes");
    } else {
      println!("No");
    }
  }
}