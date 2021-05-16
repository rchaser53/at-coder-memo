#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use std::cmp::Ordering;
use petgraph::unionfind::UnionFind;

const MOD:usize = 1_000_000_007;

fn main() {
  input!{
    n: usize,
    d: usize,
    k: usize,
    days:[(Usize1, Usize1);d],
    vals:[(Usize1, Usize1);k]
  }
  
  for (mut from, to) in vals {
    if from < to {
      for i in 0..d {
        let (l, r) = days[i];
        if l <= from && from <= r {
          from = r;
          if to <= r {
            println!("{}", i+1);
            break
          }
        }
      }
    } else {
      for i in 0..d {
        let (l, r) = days[i];
        if l <= from && from <= r {
          from = l;
          if l <= to {
            println!("{}", i+1);
            break
          }
        }
      }
    }
  }
}
