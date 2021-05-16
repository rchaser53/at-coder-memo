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
    m:usize,
    vals:[Chars;n]
  }
  let mut odd = 0usize;
  let mut even = 0;
  for v in vals {
    let mut count = 0;
    for i in 0..m {
      if v[i] == '1' {
        count += 1;
      }
    }
    if count % 2 == 1 {
      odd += 1;
    } else {
      even += 1;
    }
  }
  println!("{}", even * odd);
}