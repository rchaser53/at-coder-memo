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

#[fastout]
fn main() {
  input!{
    s:Chars
  }
  
  if s[0] != 'A' {
    println!("WA");
    return
  }
  let len = s.len();
  let mut memo = vec![];
  
  for i in 0..len {
    if 'A' <= s[i] && s[i] <= 'Z' {
      memo.push(i);
    }
  }
  
  if memo.len() != 2 {
    println!("WA");
    return
  }
  let ti = memo[1];
  
  if ti < 2 || s.len() - 1 - 1 < ti || s[ti] != 'C' {
    println!("WA");
    return
  }
  
  println!("AC");
}