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
    h:isize,
    w:isize
  }
  
  if h % 3 == 0 || w % 3 == 0 {
    println!("0");
  } else {
    let mut min = 1_000_000_000_000;
    for i in 1..h {
      let base = i*w;
      let left = h-i;
      let even = left * (w/2);
      if w % 2 == 1 {
        let odd = left * (w/2+1);
        let t_max = std::cmp::max(base, std::cmp::max(even, odd));
        let t_min = std::cmp::min(base, std::cmp::min(even, odd));
        min = std::cmp::min(min, (t_max-t_min).abs());
      } else {
        min = std::cmp::min(min, (base - left * w/2).abs());
      }
    }
    
    for i in 1..w {
      let base = i*h;
      let left = w-i;
      let even = left * (h/2);
      if h % 2 == 1 {
        let odd = left * (h/2+1);
        let t_max = std::cmp::max(base, std::cmp::max(even, odd));
        let t_min = std::cmp::min(base, std::cmp::min(even, odd));
        min = std::cmp::min(min, (t_max-t_min).abs());
      } else {
        min = std::cmp::min(min, (base - left * h/2).abs());
      }
    }
    
    println!("{}", std::cmp::min(h, std::cmp::min(min, w)));
  }
}