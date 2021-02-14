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
      if (i+ii) % 2 == 1 {
        v *= -1;
      }
      let temp = memo[i][ii+1] + memo[i+1][ii] - memo[i][ii] + v;
      memo[i+1][ii+1] = temp;
      for iii in 0..=i {
        for iiii in 0..=ii {
          if temp - memo[i+1][iiii] - memo[iii][ii+1] + memo[iii][iiii] == 0 {
            max = std::cmp::max(max, (i-iii+1) * (ii-iiii+1));
          }
        }
      }         
    }
  }   
  println!("{}", max);
}