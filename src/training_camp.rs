#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;
use std::collections::*;

const MOD:usize = 1_000_000_007;

fn main() {
  input! {
    x: usize,
  }
  
  let mut i = 1;
  for v in 2..=x {
    i *= v;
    i %= MOD;
  }
  println!("{}", i);
}