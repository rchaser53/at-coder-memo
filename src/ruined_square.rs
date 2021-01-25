#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use std::cmp::Ordering;
use petgraph::unionfind::UnionFind;

fn main() {
  input!{
    x1: isize,
    y1: isize,
    x2: isize,
    y2: isize
  }
  
  let xx2 = x2 - x1;
  let yy2 = y2 - y1;
  
  let xx4 = -yy2;
  let yy4 = xx2;
  let xx3 = xx4 + xx2;
  let yy3 = yy4 + yy2;
  
  println!("{} {} {} {}", xx3 + x1, yy3 + y1, xx4 + x1, yy4 + y1);
}