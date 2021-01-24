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

#[fastout]
fn main() {
  input!{
    n: usize,
    m: usize,
    vals: [[i128;3];n]
  }
    
  let mut max = 0;
  let mut pattern = vec![];
  for i in 0..8 {
    let mut p = vec![true;3];
    for ii in 0..3 {
      p[ii] = i >> ii & 1 == 1;
    }
    pattern.push(p);
  }
  
  for cmb in pattern {
    let mut memo = vals.clone();
    memo.sort_by(|a, b| {
      let mut temp_a = 0;
      let mut temp_b = 0;
      for i in 0..3 {
        if cmb[i] {
          temp_a += a[i];
          temp_b += b[i];
        } else {
          temp_a -= a[i];
          temp_b -= b[i];
        }
      }
      temp_a.cmp(&temp_b)
    });
    memo.reverse();
    
    let mut sa = 0i128;
    let mut sb = 0i128;
    let mut sc = 0i128;
    for i in 0..m {
      sa += memo[i][0];
      sb += memo[i][1];
      sc += memo[i][2];
    }
    max = std::cmp::max(max, sa.abs() + sb.abs() + sc.abs());
  }
  println!("{}", max);
}