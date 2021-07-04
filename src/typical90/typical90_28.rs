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
    vals:[(usize,usize,usize,usize);n]
  }
  
  let limit = 1010;
  let mut memo = vec![vec![0isize;limit];limit];
  for (lx, ly, rx, ry) in vals {
    for i in ly..ry {
      memo[i][lx] += 1;
      memo[i][rx] -= 1;
    }
  }
  
  let mut rows = vec![vec![0;limit];limit];
  for i in 0..limit {
    let mut base = 0;
    for j in 0..limit {
      base += memo[i][j];
      rows[i][j] = base;
    }
  }
  
  let mut result = vec![0;n+1];
  for i in 0..limit {
    for j in 0..limit {
      let v = rows[i][j];
      if 0 < v {
        result[v as usize] += 1;        
      }
    }
  }
  
  for i in 1..=n {
    println!("{}", result[i]);
  }
}