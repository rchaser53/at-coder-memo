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
use superslice::*;
use std::cmp::Ordering;
use num_complex::Complex;
 
const MOD:usize = 998244353;
const MAX:usize = 400010;
 
fn culc(mut n:usize) -> HashMap<usize, usize> {
  let mut map = HashMap::new();
  let mut i = 2;
  while i * i <= n {
    if n % i == 0 {
      n /= i;
      *map.entry(i).or_insert(0) += 1;
    } else {
      i += 1;
    }
  }

  if 1 < n {
    *map.entry(n).or_insert(0) += 1;
  }
  map
}

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
    n:usize,
    m:usize
  }
  
  if n == 1 {
    println!("{}", m);
    return
  }
  
  let mut fact:Vec<usize> = vec![0;MAX];
  let mut finv:Vec<usize> = vec![0;MAX];
  let mut inv:Vec<usize> = vec![0;MAX];
  
  init(&mut fact, &mut finv, &mut inv);
  
  let nn = n-1;
  let mut result = 0;
  for i in 1..=m {
    let map = culc(i);
    let mut temp = 1;
    for (_, num) in map {
      temp *= com(&fact, &finv, num+nn, num);
      temp %= MOD;
    }

    result += temp;
    result %= MOD;
  }
  println!("{}", result);
}