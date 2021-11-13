#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;
use std::collections::*;

fn main() {
  input! {
    n: usize,
  }
  
  if n <= 6 {
    println!("1");
  } else if n <= 11 {
    println!("2");
  } else {
    let mut v = (n / 11) * 2;
    let a = n % 11;
    if 0 < a && a <= 6 {
      v += 1;
    } else if 7 <= a {
      v += 2;
    }
    println!("{}", v);
  }
}