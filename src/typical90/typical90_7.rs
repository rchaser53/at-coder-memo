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
    mut vals:[isize;n],
    q:usize,
    queries:[isize;q]
  }
  vals.sort();
  
  for v in queries {
    let i = match vals.binary_search(&v) {
      Ok(a) => a,
      Err(a) => a
    };
    let result = std::cmp::min(
      (vals[i % n] - v).abs(),
      (vals[(i-1) % n] - v).abs()
    );
    println!("{}", std::cmp::min(
        result,
        (vals[(i+1) % n] - v).abs()
      )
    );
  }
}