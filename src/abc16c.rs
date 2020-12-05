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
    m: usize,
    vals: [(Usize1,Usize1);m]
  }
  
  let mut memo = vec![HashSet::new();n];
  for (from, to) in vals {
    memo[from].insert(to);
    memo[to].insert(from);
  }
  
  for i in 0..n {
    let mut set = HashSet::new();
    for ii in memo[i].iter() {
      let ii = *ii;
      if i == ii { continue }
      for iii in memo[ii].iter() {
        if memo[i].contains(iii) { continue }
        if i == *iii { continue }
        set.insert(*iii);
      }
    }
    println!("{}", set.into_iter().len());
  }
}