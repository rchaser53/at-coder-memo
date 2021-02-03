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
    s: Chars
  }
  
  let len = s.len();
  let vec_size = len*3;
  let mut min = len;
  let mut base = vec![false;vec_size];
  for i in 0..len {
    base[i] = s[i] == 'o';
    base[len+i] = s[i] == 'o';
    base[2*len+i] = s[i] == 'o';
  }

  let n = len-1;
  let limit = 1 << n;
  for i in 0..limit {
    let mut memo = base.clone();

    for ii in 0..n {
      if i >> ii & 1 == 1 {
        let offset = ii+1;
        for iii in 0..len {
          memo[(iii+offset) % vec_size] |= s[iii] == 'o';
          memo[(iii+offset+len) % vec_size] |= s[iii] == 'o';
          memo[(iii+offset+len*2) % vec_size] |= s[iii] == 'o';
        }
      }
    }
    
    let mut success = true;
    let mut start = false;
    for ii in 0..vec_size {
      if start {
        if !memo[ii] {
          success = false;
          break
        }
      } else {
        if memo[ii] {
          start = true;
        }
      } 
    }

    if success {
      let mut count = 1;
      for ii in 0..n {
        if i >> ii & 1 == 1 {
          count += 1;
        }
      }
      min = std::cmp::min(min, count);
    } 
  }
  
  println!("{}", min);
}