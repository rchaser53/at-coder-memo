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

fn main() {
  input!{
    n: usize,
    vals: [(usize,usize);n]
  }
  
  let mut a = vals[0].0;
  let mut b = vals[0].1;
  for i in 1..n {
    let (mut l, mut r) = vals[i];
    let ol = l;
    let or = r;

    if l + r < a + b {
      let mut v = (a+b) / (l+r);
      if (a+b) % (l+r) != 0 {
        v += 1;
      }
      l *= v;
      r *= v;
    }
    
    if l < a {
      let mut v = a / ol;
      if a % ol != 0 {
        v += 1;
      }
      a = ol * v;
      b = or * v;
    } else if r < b {
      let mut v = b / or;
      if b % or != 0 {
        v += 1;
      }
      a = ol * v;
      b = or * v;
    } else if a <= l && b <= r {
      a = l;
      b = r;
    }
  }
  
  println!("{}", a + b);
}