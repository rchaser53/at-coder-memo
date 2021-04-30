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

fn upper_bound(arr:&Vec<(usize,isize)>, x: &usize) -> usize {
  let mut low = 0;
  let mut high = arr.len();
  while low != high {
    let mid = (low + high) / 2;
    match arr[mid].0.cmp(x) {
      Ordering::Less | Ordering::Equal => {
        low = mid + 1;
      }
      Ordering::Greater => {
        high = mid;
      }
    }
  }
  
  low
}

fn main() {
  input! {
    n:usize,
    d:usize,
    a:isize,
    mut vals:[(Usize1,isize);n]
  }
  vals.sort_by(|a,b| a.0.cmp(&b.0));
  let mut ci = 0;
  let mut count = 0;
  let mut memo = vec![0;n+1];
  let mut temp = 0;
  while ci < n {
    temp += memo[ci];
    
    if vals[ci].1 <= temp {
      ci += 1;
      continue
    }
    
    let (pi, h) = vals[ci];
    let ti = upper_bound(&vals, &(pi+2*d));
    let mut c = (h - temp) / a;
    if h % a != 0 {
      c += 1;
    }
    count += c;
    let rv = c * a;
    temp += rv;
    memo[ti] -= rv;
    ci += 1;
  }
  println!("{}", count);
}