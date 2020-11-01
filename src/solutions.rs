#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;

#[fastout]
fn main() {
  input!{
    n: isize,
    m: isize,
  }
 
  if n == 1 && m == 0 {
    println!("1 2");
  } else if m < 0 || n - 2 < m {
    println!("-1");
  } else {
    for i in 0..n-1 {
      println!("{} {}", 3*i+3, 3*i+4);
    }
    println!("1 {}", 3*m+5);
  }
}