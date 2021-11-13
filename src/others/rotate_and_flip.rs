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

#[fastout]
fn main() {
  input!{
    n: usize,
    points: [(i128, i128);n],
    m: usize,
  }
  
  let mut memo_x = vec![(0, 1, 0);m+1];
  let mut memo_y = vec![(0, 0, 1);m+1];
  for i in 0..m {
    input!{
      t: usize
    }
    let v = if 3 <= t {
      input!{
        k: i128
      }
      k
    } else {
      0
    };
    
    if t == 1 {
      memo_x[i+1] = memo_y[i];
      let (a,b,c) = memo_x[i];
      memo_y[i+1] = (-a, -b, -c);
    } else if t == 2 {  
      memo_y[i+1] = memo_x[i];
      let (a,b,c) = memo_y[i];
      memo_x[i+1] = (-a, -b, -c);
    } else {
      let pt = if t == 3 {
        memo_y[i+1] = memo_y[i];
        &mut memo_x
      } else {
        memo_x[i+1] = memo_x[i];
        &mut memo_y
      };
      
      pt[i+1] = (2 * v - pt[i].0, -pt[i].1, -pt[i].2);
    }
  }
  
  input!{
    q: usize,
    queries: [(usize, Usize1);q]
  }
  
  for (a, b) in queries {
    println!("{} {}",
      memo_x[a].0 + points[b].0 * memo_x[a].1 + points[b].1 * memo_x[a].2,
      memo_y[a].0 + points[b].0 * memo_y[a].1 + points[b].1 * memo_y[a].2
    );
  }  
}