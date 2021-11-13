#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;
use std::collections::*;

fn main() {
  input! {
    n: usize,
    mut vals:[usize;n]
  }
  
  vals.sort();
  let mut min = vals[0];
  loop {
    for i in 0..n {
      if min < vals[i] {
        let v = vals[i] % min;
        if v == 0 {
          vals[i] = min;
        } else {
          vals[i] = v;
        }
      }
      min = std::cmp::min(min, vals[i]);
    }
    
    if vals[n-1] == min {
      println!("{}", min);
      return
    }
  }
}