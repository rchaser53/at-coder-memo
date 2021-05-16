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
    s: Chars,
  }
  
  let mut memo = vec![];
  let mut first = 0;
  for i in 1..s.len() {
    if s[i].is_uppercase() && first != i {
      memo.push((first, i));
      first = i+1;
    }
  }
  
  let mut result = vec![];
  for (si, ei) in memo {
    let mut temp = vec![];
    for i in si..=ei {
      temp.push(s[i].to_string());
    }
    result.push(temp.into_iter().collect::<String>());
  }
  result.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));
  println!("{}", result.join(""));
}
