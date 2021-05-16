#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use std::cmp::Ordering;
use maplit::{btreemap, btreeset, hashmap, hashset};
use petgraph::unionfind::UnionFind;
use petgraph::algo::dijkstra;
use petgraph::graph::{NodeIndex, UnGraph};
use num::Num;

const MOD:usize = 1_000_000_007;

fn main() {
  input!{
    n: usize,
    m: usize,
    mut vals: [(usize, usize);m]
  }
  
  let mut changes = vec![(0, 0);n+1];
  for i in 0..m {
    let (l, r) = vals[i];
    changes[l].0 += 1;
    changes[r].1 += 1;
  }
  let mut memo = vec![0;n+1];
  let mut temp = 0;
  for i in 0..=n {
    temp += changes[i].0;
    memo[i] += temp;
    temp -= changes[i].1;
  }
  for i in 0..=n {
    if 1 < memo[i] {
      memo[i] = 1;
    } else {
      memo[i] = 0;
    }
  }
  for i in 1..=n {
    memo[i] += memo[i-1];
  }
  
  let mut result: Vec<usize> = vec![];
  for i in 0..m {
    let (l, r) = vals[i];
    let v = memo[r] - memo[l-1];
    if v == (r+1-l) {
      result.push(i+1);
    }
  }

  println!("{}", result.len());
  for v in result {
    println!("{}", v);
  }
}
