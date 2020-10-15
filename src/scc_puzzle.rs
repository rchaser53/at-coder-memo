#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;
use std::collections::*;

fn main() {
  input! {
    n: usize,
    mut m: usize
  }
  
  if m == n * 2 {
    println!("{}", n);
  } else if m < n * 2 {
    println!("{}", m / 2); 
  } else {
    m -= n * 2;
    println!("{}", m / 4 + n);
  }
}