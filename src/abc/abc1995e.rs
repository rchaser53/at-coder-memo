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
use num_complex::Complex;
use nalgebra::{Rotation2, Vector2};
use std::collections::*;
use std::cmp::Ordering;
use std::cmp::Reverse;

const MOD:usize = 1_000_000_007;
const MAX: usize = 1000;

fn main() {
  input!{
    r:usize,
    c:usize,
    vals:[[usize;c-1];r],
    vals2:[[usize;c];r-1]
  }  
  let mut memo = vec![vec![1_000_000;c];r*2];
  memo[0][0] = 0;
  let mut que = BinaryHeap::new();
  que.push((Reverse(0),0,0));  
  while !que.is_empty() {
    let (Reverse(v), x, y) = que.pop().unwrap();
    
    let mut push = |nv:usize, nx:usize, ny:usize| {
      if nv < memo[ny][nx] {
        que.push((Reverse(nv), nx, ny));
        memo[ny][nx] = nv;
      }
    };
    
    if r <= y {
      if r < y {
        push(v+1, x, y-1);
      }
      push(v, x, y-r);
    } else {
      if x < c-1 {
        push(vals[y][x] + v, x+1, y);
      }
      if 0 < x {
        push(vals[y][x-1] + v, x-1, y);
      }
      if y < r-1 {
        push(vals2[y][x] + v, x, y+1);
      }
      if 0 < y {
        push(v+1, x, y+r);
      } 
    }
  }  
  println!("{}", memo[r-1][c-1]);
}