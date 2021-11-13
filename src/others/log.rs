#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use std::cmp::Ordering;
use petgraph::unionfind::UnionFind;

const MOD:usize = 1_000_000_007;

fn main() {
  input!{
    n: u128,
  }
  
  if n <= 2 {
    println!("1");
    return
  }
  
  let mut l = 1;
  let mut r = n;
  loop {
    if r <= l+1 {
      let v = r * (r + 1) / 2;      
      if n + 1 < v {
        println!("{}", n - l + 1);
      } else {
        println!("{}", n - r + 1);
      }
      return
    }
    
    let middle = (l+r) / 2;
    let v = middle * (middle + 1) / 2;
    if n + 1 < v {
      r = middle;
    } else {
      l = middle;
    }
  }
}
