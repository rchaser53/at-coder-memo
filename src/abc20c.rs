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
use std::cmp::Reverse;

const MOD:usize = 1_000_000_007;
const MAX: usize = 1000;

fn culc(
  color: char,
  bc: usize,
  val: usize
) -> usize {
  let cost = if color == '#' {
    bc 
  } else {
    1
  };
  val + cost
}

fn main() {
  input! {
    h:usize,
    w:usize,
    t:usize,
    rows:[Chars;h]
  }
  
  let inf = 1_000_000_000_000_000;
  let mut left = 0;
  let mut right = inf;
  let mut start = (0,0);
  let mut goal = (0,0);
  
  for i in 0..h {
    for ii in 0..w {
      if rows[i][ii] == 'S' {
        start = (i, ii);
      } else if rows[i][ii] == 'G' {
        goal = (i, ii);
      }
    }
  }
  
  while left + 1 < right {
    let mid = (left + right) / 2;
    let mut memo = vec![vec![inf;w];h];
    memo[start.0][start.1] = 0;
    
    let mut stack = vec![start];
    while !stack.is_empty() {
      let (r, c) = stack.pop().unwrap();
      let v = memo[r][c];
      if 0 < r {
        let nv = culc(rows[r-1][c], mid, v);
        if nv < memo[r-1][c] {
          memo[r-1][c] = nv;
          stack.push((r-1, c));
        }
      }
      
      if r < h - 1 {
        let nv = culc(rows[r+1][c], mid, v);
        if nv < memo[r+1][c] {
          memo[r+1][c] = nv;
          stack.push((r+1, c));
        }
      }
      
      if 0 < c {
        let nv = culc(rows[r][c-1], mid, v);
        if nv < memo[r][c-1] {
          memo[r][c-1] = nv;
          stack.push((r, c-1));
        }
      }
      
      if c < w - 1 {
        let nv = culc(rows[r][c+1], mid, v);
        if nv < memo[r][c+1] {
          memo[r][c+1] = nv;
          stack.push((r, c+1));
        }
      }
    }
    
    let result = memo[goal.0][goal.1];
    if t < result {
      right = mid;
    } else {
      left = mid;
    }
  }
  
  println!("{}", left);
}