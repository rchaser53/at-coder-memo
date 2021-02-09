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
    first: String,
    last: String,
    n: usize,
    vals: [String;n]
  }
  if &first == &last {
    println!("0");
    println!("{}", &first);
    println!("{}", &last);
    return
  }
  
  let words = std::iter::once(first)
               .chain(vals.into_iter())
               .chain(std::iter::once(last))
               .collect_vec();
  let n = words.len();
  let mut g = vec![vec![];n];
  for i in 0..n {
    for ii in i+1..n {
      if words[i]
           .chars()
           .zip(words[ii].chars())
           .filter(|(a,b)| a != b)
           .count() == 1 {
        g[i].push(ii);
        g[ii].push(i);
      }
    }
  }
    
  let mut visited = vec![false;n];
  let mut que = VecDeque::new();
  que.push_back((0, vec![0]));
  while let Some((now, hist)) = que.pop_front() {
    if now == n - 1 {
      println!("{}", hist.len() - 2);
      for i in hist {
        println!("{}", words[i]);
      }
      return
    }
    
    for &next in &g[now] {
      if !visited[next] {
        visited[next] = true;
        let mut hist = hist.clone();
        hist.push(next);
        que.push_back((next, hist));
      }
    }
  }
  println!("-1");
}