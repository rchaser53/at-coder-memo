#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use petgraph::unionfind::UnionFind;

#[fastout]
fn main() {
  input!{
    n:usize,
    vals:[usize;n]
  }
  
  let mut set = HashSet::new();
  set.insert(0);
  for v in vals {
    let mut temp = vec![v;set.len()];
    let mut i = 0;
    for vv in set.iter() {
      temp[i] += vv;
      i += 1;
    }
    for v in temp {
      set.insert(v);
    }
  }
  println!("{}", set.len());
}