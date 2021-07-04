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
    a:i128,
    b:u32,
    c:i128
  }
  if a < c.pow(b) {
    println!("Yes");
  } else {
    println!("No");
  }
}
