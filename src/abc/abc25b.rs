#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use std::cmp::Ordering;
use petgraph::unionfind::UnionFind;

const MOD:usize = 1_000_000_007;

fn main() {
  input!{
    n: usize,
    a: isize,
    b: isize,
    vals:[(String,isize);n]
  }
  
  let mut current = 0;
  for (d, v) in vals {
    let nv = if v <= a {
      a
    } else if b <= v {
      b
    } else {
      v
    };
    
    if d == String::from("East") {
      current += nv;
    } else {
      current -= nv;
    }
  }
  
  if current == 0 {
    println!("0");
  } else if 0 < current {
    println!("East {}", current);
  } else {
    println!("West {}", current.abs());
  }
}
