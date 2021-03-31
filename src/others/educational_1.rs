// https://github.com/E869120/kyopro_educational_90/blob/main/problem/001.jpg

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

const MOD:usize = 998244353;
const MAX:usize = 200_000;

fn main() {
  input!{
    n:usize,
    l:usize,
    k:usize,
    mut vals:[usize;n]
  }
  vals.push(l);
  
  let mut min = 1;
  let mut max = l;
  while min + 1 < max {
    let mid = (min+max) / 2;
    let mut last = 0;
    let mut count = 0;
    let mut i = 0;
    let mut temp = 0;
    while i <= n {
      if mid <= temp {
        temp = 0;
        count += 1;
      } else {
        temp += vals[i] - last;
        last = vals[i];
        i += 1;
      }
    }

    if mid <= temp {
      count += 1;
    }
    
    if count <= k {
      max = mid;
    } else {
      min = mid;
    }
  }
  println!("{}", min);
}