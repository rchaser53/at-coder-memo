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

fn main() {
  input!{
    n: usize,
    s: Chars
  }
  
  let mut que = VecDeque::new();
  let mut temp = None;
  for c in s {
    if que.is_empty() {
      que.push_back(c);
      continue
    }
    
    if que.front().unwrap() == &c {
      que.pop_front().unwrap();
      if let Some(v) = temp {
        que.push_back(v);
        temp = None;
      }
      continue
    }
    
    if que.back().unwrap() == &c {
      que.pop_back().unwrap();
      if let Some(v) = temp {
        que.push_back(v);
        temp = None;
      }
      continue
    }
    
    if que.len() == 1 {
      que.push_back(c);
      continue
    }
    
    if let Some(v) = temp {
      temp = None;
    } else {
      temp = Some(c);
    }
  }
  
  let mut result = que.len();
  if let Some(v) = temp {
    result += 1;
  }
  println!("{}", result);
}