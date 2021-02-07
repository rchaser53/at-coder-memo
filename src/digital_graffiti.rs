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
    h: usize,
    w: usize,
    rows: [Chars;h]
  }

  let mut count = 0;
  for i in 0..h-1 {
    for ii in 0..w-1 {
      let mut temp = 0;
      for iii in 0..2 {
        for iiii in 0..2 {
          if rows[i+iii][ii+iiii] == '#' {
            temp += 1;
          }
        }
      }
      if temp == 1 || temp == 3 {
        count += 1;
      }
    }
  }
 
  println!("{}", count);
}