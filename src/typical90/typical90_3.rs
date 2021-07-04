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
use std::cmp::Ordering;
use num_complex::Complex;
 
const MOD:usize = 998244353;
const MAX:usize = 400010;

fn main() {
  input!{
    n:usize,
    vals:[(Usize1,Usize1);n-1]
  }
  let mut memo = vec![];
  for (from, to) in vals {
    memo.push((from, to, 1));
  }
  let g = UnGraph::<usize, usize, usize>::from_edges(&memo);
  let res = dijkstra(&g, 0.into(), Some(n.into()), |e| *e.weight());
  let mut max = 0;
  let mut ti = 0;
  for i in 0..n {
    let v = *res.get(&NodeIndex::new(i)).unwrap();
    if max < v {
      ti = i;
      max = v;
    }
  }
  
  let res = dijkstra(&g, ti.into(), Some(n.into()), |e| *e.weight());
  let mut max = 0;
  for i in 0..n {
    max = std::cmp::max(max, *res.get(&NodeIndex::new(i)).unwrap());
  }
  println!("{}", max+1);
}
