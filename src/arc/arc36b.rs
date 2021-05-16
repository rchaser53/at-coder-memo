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
    n:usize,
    vals:[usize;n]
  }
  let mut left = 0;
  let mut middle = 0;
  let mut right = 0;
  let mut max = 1;
  while left < n {
    let mut middle = left+1;
    while middle < n {
      if vals[middle-1] < vals[middle] {
        middle += 1;
      } else {
        break
      }
    }
    middle -= 1;

    if middle == n-1 {
      max = std::cmp::max(max, middle-left+1);
      break
    }
    
    let mut right = middle+1;
    while right < n {
      if vals[right] < vals[right-1] {
        right += 1;
      } else {
        break
      }
    }
    right -= 1;
    max = std::cmp::max(max, right-left+1);
    if right == n-1 { break }
    if left == right {
      left += 1;
    } else {
      left = right;
    }
    
  }
  
  println!("{}", max); 
}