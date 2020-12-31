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

fn main() {
  input!{
    n: u32,
    vals: [usize;2usize.pow(n)]
  }
  let mut que = VecDeque::with_capacity(2usize.pow(n));
  let mut memo = vec![0;2usize.pow(n)];  
  for i in 0..vals.len() {
    que.push_back((i, vals[i]));
  }
  let mut count = 1;
  while 2 < que.len() {
    let mut que_t = VecDeque::with_capacity(
      2usize.pow(n.saturating_sub(count) as u32)
    );
    while let Some((i, v)) = que.pop_front() {
      if let Some((i2, v2)) = que.pop_front() {
        if v < v2 {
          que_t.push_back((i2, v2));
          memo[i] = count;
        } else {
          que_t.push_back((i, v));
          memo[i2] = count;
        }
      } else {
        unreachable!();
      }
    }
    que = que_t;
    count += 1;
  } 
  
  for (i, _) in que {
    memo[i] = count;
  }
  
  for v in memo {
    println!("{}", v);
  }
}


