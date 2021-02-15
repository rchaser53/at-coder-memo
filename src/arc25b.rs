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

const MOD:usize = 1_000_000_007;

#[fastout]
fn main() {
  input!{
    h: usize,
    w: usize,
    mut rows: [[isize;w];h]
  }
  
  let mut max = 0;
  let mut memo = vec![vec![0;w+1];h+1];
  for i in 0..h {
    for ii in 0..w {
      let mut v = rows[i][ii];
      if (i+ii) % 2 == 0 {
        v *= -1;
      }
      memo[i+1][ii+1] = v + memo[i+1][ii] + memo[i][ii+1] - memo[i][ii];
      
      for r in 0..=i {
        for c in 0..=ii {
          let v = memo[i+1][ii+1] + memo[r][c] - memo[i+1][c] - memo[r][ii+1];
          if v == 0 {
            max = std::cmp::max(max, (i-r+1) * (ii-c+1));
          }
        }
      }
    }
  }
  println!("{}", max);
}