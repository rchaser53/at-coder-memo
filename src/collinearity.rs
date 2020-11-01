#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use petgraph::unionfind::UnionFind;

#[fastout]
fn main() {
  input!{
    n: usize,
    vals: [(isize, isize);n]
  }
  
  for i in 0..n {
    let (x1, y1) = vals[i];
    for ii in i+1..n {
      let (x2, y2) = vals[ii];
      for iii in ii+1..n {
        let (x3, y3) = vals[iii];
        if (x2 - x1) * (y3 - y1) == (y2 - y1) * (x3 - x1) {
          println!("Yes");
          return
        }
      }
    }
  }
  println!("No");
}