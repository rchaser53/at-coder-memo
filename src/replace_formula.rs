#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use std::cmp::Ordering;
use petgraph::unionfind::UnionFind;

fn main() {
  input!{
    s: String
  }
  
  let mut count = 0;
  let mut vals = s.split('+').collect::<Vec<&str>>();
  for i in 0..vals.len() {
    let mut exist_zero = false;
    for c in vals[i].chars() {
      if c == '0' {
        exist_zero = true;
        break
      }
    }
    if !exist_zero {
      count += 1;
    }
  }
  println!("{}", count);  
}
