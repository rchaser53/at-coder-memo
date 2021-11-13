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
    d: usize,
    g: isize,
    vals: [(isize, isize);d]
  }
  
  let mut min = 10_000;
  let limit = 1 << d;
  for i in 0..limit {
    let mut lefts = vec![true;d];
    let mut total = 0;
    let mut count = 0;
    for ii in 0..d {
      if i >> ii & 1 == 1 {
        lefts[ii] = false;
        let v = ((ii + 1) * 100) as isize;
        total += vals[ii].0 * v + vals[ii].1;
        count += vals[ii].0;
      }
    }
    
    if total < g {
      let mut que = vec![];
      for ii in 0..d {
        if lefts[ii] {
          let v = ((ii + 1) * 100) as isize;
          que.push((vals[ii].0, vals[ii].1, v));
        }
      }
      
      while total < g {
        let li = que.len()-1;
        que[li].0 -= 1;
        count += 1;
        total += que[li].2;
        if que[li].0 == 0 {
          total += que[li].1;
          que.pop();
        }
      }
    }
    min = std::cmp::min(min, count);
  }
  
  println!("{}", min);
}