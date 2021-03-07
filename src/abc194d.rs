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
    un: usize
  }
  let n = un as f64;
  let mut result = 0f64;
  for i in 1..un {
    let v = i as f64;
    result += n / (n - v);
  }
  println!("{}", result);
}
  