#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use std::cmp::Ordering;
use maplit::{btreemap, btreeset, hashmap, hashset};
use petgraph::unionfind::UnionFind;
use petgraph::algo::dijkstra;
use petgraph::graph::{NodeIndex, UnGraph};

const MOD:usize = 1_000_000_007;

fn main() {
  input!{
    n: usize,
    k: usize,
    mut vals:[[usize;k];n]
  }
  
  for i in 0..n {
    vals[i].sort();
  }
  
  let mut lasts = vec![0;k];
  for i in 0..k {
    lasts[i] = k-i;
  }
  
  for i in (0..n-1).rev() {
    let mut ci = k-1;
    let v = vals[i][ci];
    let ti = match vals[i+1].binary_search(&v) {
      Ok(v) => v,
      Err(v) => v
    };
    
    let mut currents = vec![0;k];
    if k != ti {
      currents[ci] = lasts[ti];
    }
    ci -= 1;    

    loop {
      let v = vals[i][ci];
      let ti = match vals[i+1].binary_search(&v) {
        Ok(v) => v,
        Err(v) => v
      };
      
      if k != ti {
        currents[ci] = lasts[ti];
      }
      currents[ci] += currents[ci+1];
      currents[ci] %= MOD;

      if ci == 0 {
        break
      }
      ci -= 1;
    }
    lasts = currents;
  }
  
  println!("{}", lasts[0]);
}