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
    n:usize,
    vals:[[usize;n];n],
    m:usize,
    info:[(Usize1,Usize1);m]
  }
  
  let mut outs = vec![HashSet::new();n];
  for (a, b) in info {
    outs[a].insert(b);
    outs[b].insert(a);
  }
  
  let mut min = 1_000_000_000;
  let mut list = (0..n).collect::<Vec<usize>>();
  heap_recursive(&mut list, |p| {
    let mut temp = 0;
    for i in 0..n-1 {
      let now = p[i];
      let next = p[i+1];
      if outs[now].contains(&next) {
        return
      }
      temp += vals[now][i];
    }
    let last = p[n-1];
    temp += vals[last][n-1];
    min = std::cmp::min(min, temp);
  });
  if min == 1_000_000_000 {
    println!("-1");
  } else {
    println!("{}", min);
  }
}
