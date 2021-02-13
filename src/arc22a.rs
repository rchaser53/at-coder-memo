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
  input! {
    s: Chars
  }
  
  let mut is = vec![];
  let mut cs = vec![];
  let mut ts = vec![];
  
  for i in 0..s.len() {
    let c = s[i];
    if c == 'i' || c == 'I' {
      is.push(i);
    } else if c == 'c' || c == 'C' {
      cs.push(i);
    } else if c == 't' || c == 'T' {
      ts.push(i);
    }
  }
  
  for i in &is {
    for ii in &cs {
      for iii in &ts {
        if i < ii && ii < iii {
          println!("YES");
          return
        }
      }
    }
  }
  println!("NO");
}