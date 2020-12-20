#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use std::cmp::Ordering;
use maplit::{btreemap, btreeset, hashmap, hashset};
use petgraph::unionfind::UnionFind;
use petgraph::algo::dijkstra;
use petgraph::graph::{NodeIndex, DiGraph, UnGraph};
use permutohedron::{Heap, heap_recursive};

const MOD:usize = 1_000_000_000;

fn main() {
  input!{
    n: usize,
    m: usize,
    q: usize,
    nodes: [(Usize1, Usize1);m],
    mut colors: [usize; n],
  }
  
  let mut adjusts = vec![vec![];n];
  for (from, to) in nodes {
    adjusts[from].push(to);
    adjusts[to].push(from);
  }
  
  for _ in 0..q {
    input! {
      t: usize
    }
    
    if t == 1 {
      input! {
        x: Usize1
      }
      
      let color = colors[x];
      println!("{}", color);
      for i in 0..adjusts[x].len() {
        let target = adjusts[x][i];
        colors[target] = color;
      }
    } else {
      input! {
        x: Usize1,
        y: usize
      }
      
      let color = colors[x];
      println!("{}", color);
      colors[x] = y;
    }
  }
}
