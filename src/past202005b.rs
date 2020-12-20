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
    q: usize
  }
  
  let mut problems = vec![n;m];
  let mut men = vec![vec![];n];
  for i in 0..q {
    input! {
      t: usize
    }
    
    if t == 1 {
      input! {
        i: Usize1
      }
      let mut sum = 0;
      for ii in 0..men[i].len() {
        sum += problems[men[i][ii]];
      }
      println!("{}", sum);
    } else {
      input! {
        hi: Usize1,
        i: Usize1,
      }
      men[hi].push(i);
      problems[i] = problems[i].saturating_sub(1);
    }
  }
}