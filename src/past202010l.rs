#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use std::cmp::Ordering;
use maplit::{btreemap, btreeset, hashmap, hashset};
use petgraph::unionfind::UnionFind;
use petgraph::algo::dijkstra;
use petgraph::graph::{NodeIndex, DiGraph, UnGraph};
use permutohedron::{Heap, heap_recursive};

const MOD:usize = 1_000_000_000;

fn main() {
  input!{
    n: usize,
    q: usize,
    mut vals: [isize;n],
  }
  
  let mut mp = HashMap::new();
  for (i, h) in vals.windows(2).enumerate() {
    let h = if i % 2 == 0 { h[1] - h[0] } else { h[0] - h[1] };
    *mp.entry(h).or_insert(0) += 1;
  }
  
  let mut base = 0;
  for i in 0..q {
    input! { t: usize }
    if t == 1 {
      input! { v: isize }
      base += v;
    } else if t == 2 {
      input! { v: isize }
      base -= v;
    } else {
      input! { ii: Usize1, v: isize }
      if let Some(s) = vals.get(ii..=ii+1) {
        let last = if ii % 2 == 0 { s[1] - s[0] } else { s[0] - s[1] };
        *mp.get_mut(&last).unwrap() -= 1;
        let new = if ii % 2 == 0 {
          s[1] - s[0] - v
        } else {
          s[0] + v - s[1]
        };
        *mp.entry(new).or_insert(0) += 1;
      }
      if let Some(s) = vals.get(ii.wrapping_sub(1)..=ii) {
        let last = if (ii-1) % 2 == 0 { s[1] - s[0] } else { s[0] - s[1] };
        *mp.get_mut(&last).unwrap() -= 1;
        let new = if (ii-1) % 2 == 0 {
          s[1] + v - s[0]
        } else {
          s[0] - s[1] - v
        };
        *mp.entry(new).or_insert(0) += 1;
      }
      vals[ii] += v;
    }
    println!("{}", mp.get(&base).unwrap_or(&0));
  }
}
