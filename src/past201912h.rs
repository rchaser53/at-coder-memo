#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use std::cmp::*;
use maplit::{btreemap, btreeset, hashmap, hashset};
use petgraph::unionfind::UnionFind;
use petgraph::algo::dijkstra;
use petgraph::graph::{NodeIndex, DiGraph, UnGraph};
use permutohedron::{Heap, heap_recursive};

const MOD:usize = 1_000_000_007;

fn main() {
  input!{
    n: usize,
    mut vals: [usize;n],
    q: usize,
  }
  
  let start = vals.iter().sum::<usize>();
  
  let mut odds_min = 1_000_000_001;
  let mut all_min = 1_000_000_001;
  for i in 0..n {
    if i % 2 == 0 {
      odds_min = std::cmp::min(odds_min, vals[i]);
    }
    all_min = std::cmp::min(all_min, vals[i]);
  }

  let n_even = n / 2 + if n % 2 == 0 { 0 } else { 1 };
  let mut count_all = 0;
  let mut count_odd = 0;
  let mut count_sum = 0;
  for i in 0..q {
    input!{
      t: usize
    }
    
    match t {
      1 => {
        input!{ ti:Usize1, val: usize }
        let now = vals[ti] - if ti % 2 == 0 {
          count_odd
        } else {
          0
        } - count_all;
        if val <= now {
          vals[ti] -= val;
          count_sum += val;
          if ti % 2 == 0 {
            odds_min = std::cmp::min(now-val, odds_min);
            all_min = std::cmp::min(all_min, odds_min);
          } else {
            all_min = std::cmp::min(now-val, all_min);
          }
        }
      },
      2 => {
        input!{ val: usize }
        if val <= odds_min {
          odds_min -= val;
          all_min = std::cmp::min(all_min, odds_min);
          count_odd += val;
          count_sum += val * n_even;
        }
      },
      _ => {
        input!{ val: usize }
        if val <= all_min {
          all_min -= val;
          odds_min -= val;
          count_all += val;
          count_sum += val * n;
        }
      }
    }
  }
  println!("{}", count_sum); 
}
