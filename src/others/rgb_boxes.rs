#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use std::cmp::Ordering;
use petgraph::unionfind::UnionFind;

#[fastout]
fn main() {
  input!{
    r: usize,
    g: usize,
    b: usize,
    n: usize,
  }
  
  let mut count = 0;
  for i in 0..=3000 {
    for ii in 0..=3000 {
      if n < i*r + ii*g { continue }
      if (n - (i*r + ii*g)) % b == 0 {
        count += 1;
      }
    }
  }
  println!("{}", count);
}