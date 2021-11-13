#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use petgraph::unionfind::UnionFind;

fn main() {
  input!{
    x: usize,
  }
  
  let mut count = 3;
  loop {
    if x * count % 360 == 0
    {
      println!("{}", count);
      return
    }
    count += 1;
  }
}