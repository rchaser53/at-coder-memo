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

fn main() {
  input!{
    m: usize,
    n: usize,
    mut v: usize
  }
  
  let mut a = 0;
  let mut total = v;
  loop {
    let mut nv = v / m;
    a += v % m;
    if a / m != 0 {
      a %= m;
      nv += 1;
    }
    if 0 < nv {
      total += n * nv;
      v = n * nv;
    } else {
      break
    }
  }
  println!("{}", total);
}