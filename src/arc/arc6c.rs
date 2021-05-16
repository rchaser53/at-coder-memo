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
    vals: [usize;n]
  }
  
  let mut stacks = vec![vec![vals[0]]];
  for i in 1..n {
    let v = vals[i];
    let mut ti = None;
    let mut mv = 1_000_000_000;
    for ii in 0..stacks.len() {
      let lv = *stacks[ii].last().unwrap();
      // 最後の値より大きい数の中で最小
      if v <= lv && lv <= mv {
        mv = lv;
        ti = Some(ii);
      }
    }
    
    if let Some(ii) = ti {
      stacks[ii].push(v);
    } else {
      stacks.push(vec![v]);
    }
  }
  println!("{}", stacks.len());
}