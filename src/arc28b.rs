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


#[fastout]
fn main() {
  input!{
    n: usize,
    k: usize,
    vals: [usize;n]
  }
  
  let mut map = HashMap::new();
  for i in 0..n {
    map.insert(vals[i], i+1);
  }
  
  let mut que = BinaryHeap::new();
  for i in 0..k {
    que.push(vals[i]);
  }
  
  for i in k..n {
    let v = que.peek().unwrap();
    println!("{}", map.get(&v).unwrap());
    que.push(vals[i]);
    que.pop();
  }

  let v = que.peek().unwrap();
  println!("{}", map.get(&v).unwrap());
}