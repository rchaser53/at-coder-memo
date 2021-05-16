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
    s: Bytes,
    mut x: Usize1
  }
  
  let sup = x + 2;
  let mut len = vec![0;s.len()+1];
  for (i, &c) in s.iter().enumerate() {
    len[i+1] = if b'a' <= c && c <= b'z' {
      std::cmp::min(sup, len[i] + 1)
    } else {
      let k = (c - b'0') as usize;
      std::cmp::min(sup, len[i] * (k + 1))
    };
  }
  for i in (1..=s.len()).rev() {
    let c = s[i-1];
    if b'1' <= c && c <= b'9' {
      x %= len[i-1];
    } else if x == len[i] - 1 {
      println!("{}", c as char);
      return
    }
  }
}