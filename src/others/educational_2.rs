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

#[fastout]
fn main() {
  input!{
    n:usize,
  }

  if n % 2 == 1 {
    println!("");
    return
  }
  
  let mut set = HashSet::new();
  let mut limit = 1 << n;
  for i in 0..limit {
    let mut temp = vec![false;n];
    let mut count = 0;
    for ii in 0..n {
      if i >> ii & 1 == 1 {
        temp[ii] = true;
        count += 1;
      } else {
        count -= 1;
      }
      if count < 0 { break }
    }
    
    if count == 0 {
      let mut result = String::from("");
      for v in temp {
        if v {
          result = format!("{}(", result);
        } else {
          result = format!("{})", result);
        }
      }
      set.insert(result);
    }

  }
  let mut result = set.into_iter().collect::<Vec<String>>();
  result.sort();
  for v in result {
    println!("{}", v);
  }
}