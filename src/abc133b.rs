#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use std::mem;

const MOD:usize = 1_000_000_007;
#[fastout]
fn main() {
  input!{
    n: usize,
    d: usize,
    vals: [[isize;d];n]
  }
  
  let mut set = HashSet::new();
  for i in 0..=1000 {
    set.insert(i * i);
  }
  
  let mut count = 0;
  for i in 0..n {
    for ii in i+1..n {
      let mut total = 0;
      for iii in 0..d {
        total += (vals[i][iii] - vals[ii][iii]).pow(2);
      }
      if set.contains(&total) {
        count += 1;
      }
    }
  }
  println!("{}", count);
}