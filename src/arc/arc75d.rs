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
    a:usize,
    b:usize,
    mut vals:[usize;n]
  }
  
  if n == 1 {
    let mut count = vals[0] / a;
    if vals[0] % a != 0 {
      count += 1;
    }
    println!("{}", count);
    return
  }
  
  vals.sort();
  vals.reverse();

  let mut min = 1;
  let mut max = 1_000_000_000;
  while min + 1 < max {
    let mid = (min+max) / 2;
    let mut count = 0;
    let v = mid * b;
    let ab = a - b;
    
    for i in 0..n {
      if vals[i] <= v {
        break
      }
      let diff = vals[i] - v;
      count += diff / ab;
      if diff % ab != 0 {
        count += 1;
      }
    }
    
    if count <= mid {
      max = mid;
    } else {
      min = mid;
    }
  }
  println!("{}", max);
}