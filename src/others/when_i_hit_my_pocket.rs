#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use std::cmp::Ordering;
use petgraph::unionfind::UnionFind;

#[fastout]
fn main() {
  input!{
    mut k: usize,
    a: usize,
    b: usize
  }
  k += 1;
  if b <= a || k <= a {
    println!("{}", k);
  } else {
    let v = a + (k - a) / 2 * (b - a) + (k - a) % 2;
    println!("{}", std::cmp::max(k, v));
  }
}