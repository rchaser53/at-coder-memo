#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use std::cmp::Ordering;
use petgraph::unionfind::UnionFind;

const MOD:usize = 1_000_000_007;

fn main() {
  input!{
    vals: [usize;5]
  }
  
  let mut set = HashSet::new();
  for i in 0..5 {
    let a = vals[i];
    for ii in i+1..5 {
      let b = vals[ii];
      for iii in ii+1..5 {
        let c = vals[iii];
        set.insert(a + b + c);
      }
    }
  }
  
  let mut result = set.into_iter().collect::<Vec<usize>>();
  result.sort();
  result.reverse();
  println!("{}", result[2]);
}
