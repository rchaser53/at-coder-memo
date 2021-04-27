#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use std::cmp::Ordering;
use petgraph::unionfind::UnionFind;
 
#[fastout]
fn main() {
  input!{
    n:usize,
    m:usize,
  }
  
  let mut memo = vec![];
  for _ in 0..n {
    input!{
      k:usize,
      vals:[usize;k]
    }
    memo.push(vals);
  }

  input!{
    p:usize,
    q:usize,
    vals:[usize;p]
  }
  
  let mut set = HashSet::new();
  for v in vals {
    set.insert(v);
  }
  
  let mut count = 0;
  for i in 0..n {
    let mut temp = 0;
    for j in 0..memo[i].len() {
      let v = memo[i][j];
      if set.contains(&v) {
        temp += 1;
      }
    }
    
    if q <= temp {
      count += 1;
    }
  }
  println!("{}", count);
}