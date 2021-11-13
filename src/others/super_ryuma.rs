#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use std::cmp::Ordering;
use petgraph::unionfind::UnionFind;

fn main() {
  input!{
    r1: isize,
    c1: isize,
    r2: isize,
    c2: isize,
  }
  
  let x = r2 - r1;
  let y = c2 - c1;
  let v = x.abs() + y.abs();
  let diff = (x - y).abs();
  
  if x == 0 && y == 0 {
    println!("0");
    return
  }

  if r1 + c1 == r2 + c2
    || r1 - c1 == r2 - c2
    || (r1-r2).abs() + (c1-c2).abs() <= 3 {
      println!("1");
      return
  }
  
  if ((r1 + c1) - (r2 + c2)).abs() <= 3
    || ((r1 - c1) - (r2 - c2)).abs() <= 3
    || (r1-r2).abs() + (c1-c2).abs() <= 6
    || (r1+c1) % 2 == (r2+c2) % 2 {
      println!("2");
      return
  }
  
  println!("3");
}
