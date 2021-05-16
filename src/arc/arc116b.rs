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
const MAX: usize = 1000;
 
fn main() {
  input!{
    n:usize,
    mut vals:[usize;n]
  }
  
  if n == 1 {
    println!("{}", vals[0] * vals[0] % MOD);
    return
  }
  
  vals.sort();
  vals.reverse();

  let mut result = vals[0] * vals[0] % MOD;
  let mut memo = 0;
  for i in 1..n {
    let v = (vals[i] + vals[i-1]) % MOD;
    memo = memo * 2 % MOD;
    let nv = (v + memo) % MOD;
    let nnv = nv * vals[i] % MOD;
    result += nnv;
    result %= MOD;
    
    memo += vals[i-1];
    memo %= MOD;
  }
  println!("{}", result);  
}