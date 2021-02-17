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
use superslice::Ext;

const MOD:usize = 1_000_000_007;

struct Helper {
  memo: HashSet<usize>,
  edges: Vec<Vec<usize>>,
  set: HashSet<usize>,
  seen: HashSet<usize>
}

impl Helper {
  fn dfs(
    &mut self,
    now: usize,
    mut stack: Vec<usize>
  ) {
    if self.set.len() == 0 { return }
    if self.set.contains(&now) {
      for v in stack {
        self.memo.insert(v);
      }
      self.set.remove(&now);
      stack = vec![];
    }
    
    for i in 0..self.edges[now].len() {
      let next = self.edges[now][i];
      if self.seen.contains(&next) { continue }
      self.seen.insert(next);
      let mut new_stack = stack.clone();
      new_stack.push(next);
      self.dfs(next, new_stack);
    }
  }
}

#[fastout]
fn main() {
  input!{
    n: usize,
    x: Usize1,
    vals: [usize;n],
    routes: [(Usize1,Usize1);n-1]
  }
  
  let mut edges = vec![vec![];n];
  let mut set = HashSet::new();
  let mut seen = HashSet::new();
  let mut memo = HashSet::new();
  
  seen.insert(x);
  for i in 0..n {
    if vals[i] == 1 {
      set.insert(i);
    }
  }
  for (l, r) in routes {
    edges[l].push(r);
    edges[r].push(l);
  }

  let mut helper = Helper { memo, edges, set, seen };
  helper.dfs(x, vec![]);
  
  println!("{}", helper.memo.len() * 2);
}