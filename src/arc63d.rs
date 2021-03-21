#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use itertools::Itertools;
use maplit::{btreemap, btreeset, hashmap, hashset};
use petgraph::unionfind::UnionFind;
use petgraph::algo::dijkstra;
use petgraph::graph::{NodeIndex, DiGraph, UnGraph};
use permutohedron::{Heap, heap_recursive};
use superslice::*;
use std::collections::*;
use std::cmp::*;

const MOD:usize = 1_000_000_007;

fn main() {
  input!{
    n:usize,
    t:usize,
    vals:[usize;n]
  }
  
  let mut heap = BinaryHeap::new();
  let mut m_benefit = 0;
  let mut count = 0;
  for i in (0..n).rev() {
    let nv = vals[i];
    if let Some(v) = heap.peek() {
      let v = *v;
      if nv < v {
        let benefit = v - nv;
        if m_benefit < benefit {
          m_benefit = benefit;
          count = 1;
        } else if m_benefit == benefit {
          count += 1;
        }
      }
    }
    heap.push(nv);
  }
  println!("{}", count);
}