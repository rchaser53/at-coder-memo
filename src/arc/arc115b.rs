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

fn main() {
  input!{
    n:usize,
    vals:[[isize;n];n]
  }
  
  let mut firsts = vec![(0isize, 0usize);n];

  for i in 0..n {
    firsts[i] = (vals[i][0], i);
  }
  for i in 1..n {
    let diff = vals[0][i] - vals[0][i-1];
    for ii in 1..n {
      if diff != vals[ii][i] - vals[ii][i-1] {
        println!("No");
        return
      }
    }
  }
  firsts.sort_by(|a,b| a.0.cmp(&b.0));
  for i in 1..n {
    firsts[i].0 = firsts[i].0 - firsts[0].0;
  }
  firsts[0].0 = 0;
  let mut target_row_index = firsts[0].1;
  firsts.sort_by(|a,b| a.1.cmp(&b.1));
  let a = firsts.into_iter().map(|v| v.0.to_string()).join(" ");
  let b = vals[target_row_index].clone().into_iter().map(|v| v.to_string()).join(" ");
  println!("Yes");
  println!("{}", a);
  println!("{}", b);
  
}