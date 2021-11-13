#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use std::mem;

#[fastout]
fn main() {
  input!{
    n: usize,
    k: usize
  }
  
  let mut a = k;
  for i in 0..n-1 {
    a *= (k-1);
  }
  println!("{}", a);
}