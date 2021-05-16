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
  let n = s.len();
  let can_p = n / 2;
  let mut pc = 0;
  for c in s {
    if c == 'p' {
      pc += 1;
    }
  }
  println!("{}", can_p - pc);
}