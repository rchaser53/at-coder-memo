/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::*;

fn gcd(a:usize, b:usize) -> usize {
  if b == 0 {
    a
  } else {
    gcd(b, a % b)
  }
}

fn main() {
  input! {
    t:usize,
    ndk:[(usize,usize,Usize1);t]
  }

  for (n,d,k) in ndk {
    println!("{}",  k*d%n + k * gcd(d,n) / n);
  }
}