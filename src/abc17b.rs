#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use std::cmp::Ordering;
use petgraph::unionfind::UnionFind;

const MOD:usize = 1_000_000_007;

fn main() {
  input!{
    x: Chars
  }
  
  if x.is_empty() {
    println!("YES");
    return
  }
  
  let mut i = 0;
  while i < x.len() {
    if i <= x.len() - 2
      && x[i] == 'c'
      && x[i+1] == 'h' {
      i += 2;
    } else if x[i] == 'o'
      || x[i] == 'k'
      || x[i] == 'u' {
      i += 1;
    } else {
      println!("NO");
      return
    }
  }
  println!("YES");
}