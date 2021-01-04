#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use std::cmp::*;
use maplit::{btreemap, btreeset, hashmap, hashset};
use petgraph::unionfind::UnionFind;
use petgraph::algo::dijkstra;
use petgraph::graph::{NodeIndex, DiGraph, UnGraph};
use permutohedron::{Heap, heap_recursive};

const MOD:usize = 1_000_000_007;

fn main() {
  input!{
    n: usize,
    s: Chars
  }
  
  let mut result = vec![];

  for i in 0..n {
    let nc = s[i];
    let mut temp = vec![];
    for c in result {
      if c != nc {
        temp.push(c);
      }
    }
    temp.push(nc);
    result = temp;
  }
  
  println!("{}", result.into_iter().map(|v| v.to_string()).collect::<String>());
}