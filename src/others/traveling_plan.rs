#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;
use std::collections::HashSet;

fn main() {
  input! {
    n: usize,
    mut vals: [isize;n]
  }
  
  vals.push(0);
  let mut last = 0;
  let mut total = 0;
  for v in vals.iter() {
    total += (v - last).abs();
    last = *v;
  }
  
  last = 0;
  for i in 0..n {
    let origin = (vals[i] - last).abs() + (vals[i+1] - vals[i]).abs();
    let updated = (vals[i+1] - last).abs();
    println!("{}", total - origin + updated);
    last = vals[i];
  }
}
