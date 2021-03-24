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

fn culc(
  vals: &Vec<usize>,
  n:usize
) -> HashSet<usize> {
  let mut i = 2;
  let mut set = hashset!{1};
  while i * i <= n {
    if n % i == 0 {
      set.insert(vals[i]);
      set.insert(vals[n/i]);
    }
    i += 1;
  }
  if 0 < n / i {
    set.insert(vals[n/i]);
  }
  set
}

fn main() {
  input!{
    n:usize,
  }
  
  let mut result = vec![0;n+1];
  let mut now = 1;
  result[1] = 1;
  for i in 2..=n {
    let set = culc(&result, i);
    let mut flag = true;
    for v in 1..=now {
      if !set.contains(&v) {
        result[i] = v;
        flag = false;
        break
      }
    }
    
    if flag {
      now += 1;
      result[i] = now;
    }
  }
  result.remove(0);
  println!("{}", result.into_iter().map(|v| v.to_string()).join(" "));
}