#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use std::cmp::Ordering;
use petgraph::unionfind::UnionFind;

const MOD:usize = 998244353;
#[fastout]
fn main() {
  input!{
    n: usize,
    k: usize,
    vals: [[usize;n];n]
  }
  
  let mut rows = vec![false;n];
  let mut columns = vec![false;n];
  for r in 0..n {
    for rr in 0..n {
      if r == rr { continue }
      let mut success = true;
      for c in 0..n {
        if k < vals[r][c] + vals[rr][c] {
          success = false;
          break
        }
      }
      if success {
        rows[rr] = true;
      }
    }
  }
  
  for c in 0..n {
    for cc in 0..n {
      if c == cc { continue }
      let mut success = true;
      for r in 0..n {
        if k < vals[r][c] + vals[r][cc] {
          success = false;
          break
        }
      }
      if success {
        columns[cc] = true;
      }
    }
  }
  let mut row = 0;
  let mut column = 0;
  for v in rows {
    if v { row += 1; }
  }
  for v in columns {
    if v { column += 1; }
  }
  
  let mut result = 1;
  for i in 2..=row {
    result *= i;
    result %= MOD;
  }
  for i in 2..=column {
    result *= i;
    result %= MOD;
  }
  
  println!("{}", result);
}