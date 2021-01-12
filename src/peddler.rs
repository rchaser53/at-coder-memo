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

struct Helper {
  neighbors: Vec<Vec<usize>>,
  vals: Vec<isize>,
  memo: Vec<isize>,
  result: isize
}

impl Helper {
  fn culc(
    &mut self,
    ci: usize,
    max: isize
  ) {

    for i in 0..self.neighbors[ci].len() {
      let ni = self.neighbors[ci][i];
      let nv = self.vals[ni];
      if self.memo[ni] < max {
        self.memo[ni] = max;
        self.result = std::cmp::max(self.result, max - nv);
        self.culc(ni, std::cmp::max(max, nv));        
      }
    }
  }
}

fn main() {
  input!{
    n: usize,
    m: usize,
    vals: [isize;n],
    routes: [(Usize1, Usize1);m]
  }
  let mut neighbors = vec![vec![];n];
  let mut memo = vec![true;n];
  for (a, b) in routes {
    // 小=>大ならいける。逆走させたいので大=>小
    if a > b {
      neighbors[a].push(b);
      memo[b] = false;
    } else {
      neighbors[b].push(a);
      memo[a] = false;
    }
  }
  let mut leafs = vec![];
  for i in 0..n {
    if memo[i] {
      leafs.push(i);
    }
  }
  let mut helper = Helper {
    neighbors,
    vals,
    memo: vec![-1_000_000_007;n],
    result: -1_000_000_007
  };
  
  for i in leafs {
    helper.culc(i, helper.vals[i]);
  }
  println!("{}", helper.result);
}