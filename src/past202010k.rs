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
    k: usize,
  }
  
  let mut vals = vec![];
  for _ in 0..k {
    input!{
      n: usize,
      row: [usize;n]
    }
    vals.push(row);
  }
  
  input!{
    q: usize,
    queries: [Usize1;q]
  }
  
  let mut memo = vec![0;21];
  let mut own_scores = vec![(vec![0;21],0);k];

  let mut count = 0;
  for i in 0..k {    
    let mut c = 0;
    let mut mini = vec![0;21];
    for ii in (0..vals[i].len()).rev() {
      let v = vals[i][ii];
      own_scores[i].0[v] += 1;
      
      for iii in 0..=v-1 {
        c += mini[iii];
      }
      mini[v] += 1;
    }
    own_scores[i].1 = c;
  }
  
  let mut temps = vec![0;21];
  for i in (0..q).rev() {
    let v = queries[i];
    count += own_scores[v].1;
    count %= MOD;
    
    for ii in 2..=20 {
      let vv = own_scores[v].0[ii];
      for iii in 1..=ii-1 {
        let s = vv * temps[iii] % MOD;
        count += s;
        count %= MOD;
      }
    }
    
    for ii in 1..=20 {
      temps[ii] += own_scores[v].0[ii];
      temps[ii] %= MOD;
    }
  }
  
  println!("{}", count);
}
