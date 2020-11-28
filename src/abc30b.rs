#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use std::cmp::Ordering;
use petgraph::unionfind::UnionFind;

fn main() {
  input!{
    n: f64,
    m: f64,
  }
  
  let n = n % 12f64;
  let na = n * 30f64 + m * 30f64 / 60f64;
  let ma = m * 360f64 / 60f64;
  
  if (na - ma).abs() < 360f64 - (na - ma).abs() {
    println!("{}", (na - ma).abs());
  } else {
    println!("{}", 360f64 - (na - ma).abs());
  } 
}