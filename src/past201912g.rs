#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use std::cmp::*;
use maplit::{btreemap, btreeset, hashmap, hashset};
use petgraph::unionfind::UnionFind;
use petgraph::algo::dijkstra;
use petgraph::graph::{NodeIndex, DiGraph, UnGraph};
use permutohedron::{Heap, heap_recursive};

const MOD:usize = 1_000_000_007;

fn main() {
  input!{
    n: usize
  }
  
  let mut memo = vec![vec![0;n];n];
  for i in 0..n {
    input!{
      vals:[isize;n-i-1]
    }
    for ii in 0..vals.len() {
      let a = (i+ii+1) % n;
      memo[i][a] = vals[ii];
      memo[a][i] = vals[ii];
    }
  }
  
  let mut max = -1_000_000 * 20;
  let limit = 2usize.pow(2*n as u32);
  for i in 0..limit {
    let mut group = vec![vec![];3];
    for ii in 0..n {
      let a = if i >> (2 * ii + 1) & 1 == 1 {
        2
      } else {
        0
      };
      let b = if i >> (2 * ii) & 1 == 1 {
        1
      } else {
        0
      };
      group[(a+b)%3].push(ii);
    }
    
	let mut total = 0;
    for gi in 0..3 {
      let g = &group[gi];
      for i in 0..g.len() {
        for ii in i+1..g.len() {
          total += memo[g[i]][g[ii]];
        }
      }
    }
    max = std::cmp::max(total, max);
  }
  println!("{}", max);
}