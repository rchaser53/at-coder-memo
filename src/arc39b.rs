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
const MAX: usize = 1000;

fn init(
  fact: &mut Vec<usize>,
  finv: &mut Vec<usize>,
  inv: &mut Vec<usize>
) {
  fact[0] = 1;
  fact[1] = 1;
  finv[0] = 1;
  finv[1] = 1;
  inv[0] = 0;
  inv[1] = 1;
  
  let mut i = 2;
  while i < MAX {
    fact[i] = fact[i-1] * i % MOD;
    inv[i] = MOD - inv[MOD % i] * (MOD / i) % MOD;
    finv[i] = finv[i-1] * inv[i] % MOD;
    i += 1;
  }
}

fn com(
  fact: &Vec<usize>,
  finv: &Vec<usize>,
  n: usize,
  k: usize
) -> usize {
  if n < k {
    0
  } else {
    fact[n] * (finv[k] * finv[n-k] % MOD) % MOD
  }
}

fn main() {
  input!{
    n: usize,
    k: usize
  }
  
  let mut fact:Vec<usize> = vec![0;MAX];
  let mut finv:Vec<usize> = vec![0;MAX];
  let mut inv:Vec<usize> = vec![0;MAX];
  
  init(&mut fact, &mut finv, &mut inv);
  
  if k < n {
    println!("{}", com(&fact, &finv, n+k-1, n-1));
  } else if k == n {
    println!("1");
  } else {
    let red = k % n;
    if red == 0 {
      println!("1");
    } else {
      println!("{}", com(&fact, &finv, n, red));
    }
  }
}