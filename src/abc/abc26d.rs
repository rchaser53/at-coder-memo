#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use itertools::Itertools;
use maplit::{btreemap, btreeset, hashmap, hashset};
use petgraph::unionfind::UnionFind;
use petgraph::algo::dijkstra;
use petgraph::graph::{NodeIndex, DiGraph, UnGraph};
use permutohedron::{Heap, heap_recursive};
use superslice::*;
use std::collections::*;
use std::cmp::*;

const MOD:usize = 1_000_000_007;

fn main() {
  input!{
    a:f64,
    b:f64,
    c:f64
  }
  
  let mut left = 0f64;
  let mut right = 200f64;
  while left + 0.00000000001 < right {
    let mid = (left + right) / 2f64;
    if a * mid + b * (c * mid * std::f64::consts::PI).sin() - 100f64 < 0f64 {
      left = mid;
    } else {
      right = mid;    
    }
  }
  println!("{}", left);
}