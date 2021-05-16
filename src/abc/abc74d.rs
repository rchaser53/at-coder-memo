#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use petgraph::unionfind::UnionFind;
use std::collections::{HashMap, HashSet, VecDeque};

#[fastout]
fn main() {
  input!{
    n:usize,
    vals:[[usize;n];n]
  }
  
  let mut memo = vals.clone();
  for i in 0..n {
    for j in 0..n {
      for k in 0..n {
        let nv = memo[j][i] + memo[i][k];
        if nv < memo[j][k] {
          memo[j][k] = nv;
        }
      }
    }
  }
  
  for i in 0..n {
    for j in 0..n {
      if memo[i][j] != vals[i][j] {
        println!("-1");
        return
      }
    }
  }
  
  let mut count = 0;
  for i in 0..n {
    for j in 0..n {
      let mut exist = true;
      for k in 0..n {
        if k == j || k == i { continue }
        if memo[i][j] == memo[i][k] + memo[k][j] {
          exist = false;
        }
      }
      if exist {
        count += memo[i][j];
      }
    }
  }

  println!("{}", count / 2);
}