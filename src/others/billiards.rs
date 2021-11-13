#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use petgraph::unionfind::UnionFind;

fn main() {
  input!{
    sx: f64,
    sy: f64,
    gx: f64,
    gy: f64
  }
  
  let a = (sy + gy) / (sx - gx);
  let b = sy - a * sx;
  println!("{}", -1f64 * b / a);
}