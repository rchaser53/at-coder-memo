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
    k:usize,
    vals:[usize;n]
  }
  let mut memo = vec![0;n+1];
  for i in 1..=n {
    memo[i] += memo[i-1] + vals[i-1];
  }

  let mut map = HashMap::new();
  for i in 1..=n {
    for ii in i..=n {
      let v = memo[ii] - memo[i-1];
      *map.entry(v).or_insert(0) += 1;
    }
  }
  
  let mut key_vals = map.into_iter().collect::<Vec<(usize, usize)>>();
  let mut border = 2usize.pow(60);
  let mut result = 0;
 
  while 0 < border {
    let mut stack = vec![];
    let mut count = 0;
    for i in 0..key_vals.len() {
      let (v, c) = key_vals[i];
      if v & border == border {
        stack.push((v,c));
        count += c;
      }
    }
    if k <= count {
      key_vals = stack;
      result += border;
    }
    border >>= 1;
  }
  
  println!("{}", result);
}