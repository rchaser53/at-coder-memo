#![allow(unused_imports)]
 
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::HashMap;

#[fastout]
fn main() {
  input! {
    h: usize,
    w: usize,
    mut vals: [[usize;w];h]
  }
  
  let mut result: Vec<(usize, usize, usize, usize)> = vec![];
  
  for r in 0..h-1 {
    for c in 0..w {
      if vals[r][c] % 2 == 1 {
        vals[r][c] -= 1;
        vals[r+1][c] += 1;
        result.push((r+1, c+1, r+2, c+1));
      }
    }
  }

  let r = h-1;
  for c in 0..w-1 {
    if vals[r][c] % 2 == 1 {
      vals[r][c] -= 1;
      vals[r][c+1] += 1;
      result.push((r+1, c+1, r+1, c+2));
    }
  }
  
  println!("{}", result.len());
  for (r, c, nr, nc) in result {
    println!("{} {} {} {}", r, c, nr, nc);
  }
}