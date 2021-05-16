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

fn culc(
  points: &Vec<(f64, f64, usize)>,
) -> f64 {
  let mut memo = hashset!(0);
  let mut s_sum = 0f64;
  let n = points.len();
  while memo.len() < n {
    let mut ti = 0;
    let mut temp = 100_000_000_000f64;
    for i in memo.iter() {
      let i = *i;
      let (x1, y1, t1) = points[i];
      for ii in 0..n {
        if memo.contains(&ii) { continue }
        let (x2, y2, t2) = points[ii];
        let mut new = ((x1 - x2).powi(2) + (y1 - y2).powi(2)).sqrt();
        if t1 != t2 {
          new *= 10f64;
        }
        if new < temp {
          temp = new;
          ti = ii;
        }
      }
    }
    memo.insert(ti);
    s_sum += temp;
  }
  s_sum
}

fn main() {
  input!{
    n: usize,
    m: usize,
    mut larges: [(f64, f64, usize);n],
    smalls: [(f64, f64, usize);m]
  }

  let mut min = std::f64::MAX;
  let limit = 1 << m;
  for i in 0..limit {
    let mut adj = larges.clone();
    
    for ii in 0..m {
      if i >> ii & 1 == 1 {
        adj.push(smalls[ii].clone());
      }
    }
    
    let new = culc(&adj);
    if new < min {
      min = new;
    }
  }
  
  println!("{}", min);
}