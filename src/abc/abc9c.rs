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

const MOD:usize = 998244353;
const MAX:usize = 200_000;

fn main() {
  input!{
    n:usize,
    kk:usize,
    base:Chars
  }
  let mut result = base.clone();
  for i in 0..n {
    let mut l = i;
    for j in i+1..n {
      if result[j] < result[l] {
        let count = (0..n).filter(|&k| {
            let v = if j == k {
              result[i]
            } else if i == k {
              result[j]
            } else {
              result[k]
            };
            base[k] != v
          })
          .count();
        if count <= kk {
          l = j;
        }
      }
    }
    if i < l {
      result.swap(i, l);
    }
  }
  println!("{}", result
    .into_iter()
    .map(|v| v.to_string())
    .collect::<String>()
  );
}