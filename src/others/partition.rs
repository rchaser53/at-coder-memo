#![allow(unused_imports)]
 
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::HashMap;
 
#[fastout]
fn main() {
  input! {
    n: usize,
    m: usize,
  }
  
  let mut result = 1;
  let mut v = 1;
  while v * v <= m {
    if m % v == 0 {
      let b = m / v;
      if  v * n <= m {
        result = std::cmp::max(result, v);
      }
      if b * n <= m {
        result = std::cmp::max(result, b);
      }
    }
    v += 1;
  } 
  println!("{}", result);
}