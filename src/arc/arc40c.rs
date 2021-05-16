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

#[fastout]
fn main() {
  input!{
    n: usize,
    mut rows: [Chars;n]
  }
  
  let mut count = 0;
  for r in 0..n {
    for c in (0..n).rev() {
      if rows[r][c] == '.' {
        count += 1;
        for i in 0..=c {
          rows[r][i] = 'o';
        }
        if r < n-1 {
          for i in c..n {
            rows[r+1][i] = 'o';
          }
        }
      }
    } 
  }
  println!("{}", count);
}