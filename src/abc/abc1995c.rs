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
    n:usize,
    vals:[[usize;5];n]
  }

  
  let mut left = 0;
  let mut right = 1_000_000_010;
  let perfect = (1 << 5) - 1;
  while left+1 < right {
    let mid = (left+right) / 2;
    let mut set = HashSet::new();
    
    for i in 0..n {
      let mut temp = 0;
      for j in 0..5 {
        if mid <= vals[i][j] {
          temp |= 1 << j;
        }
      }
      set.insert(temp);
    }
    
    let mut set = set.into_iter().collect::<Vec<usize>>();
    let len = set.len();
    let mut success = false;
    for i in 0..len {
      for j in 0..len {
        for k in 0..len {
          if set[i] | set[j] | set[k] == perfect {
            success = true;
          }
        }
      }
    }
    
    if success {
      left = mid;
    } else {
      right = mid;
    }
  }
  println!("{}", left);
}