#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;
use std::collections::VecDeque;

fn main() {
  input!{
    n: usize,
    k: usize
  }
  
  let mut result = 0;
  for b in 1..=n {
    let v = if b < k {
      0
    } else {
      b - k
    };
    let p = n % b;
    let pv = if p + 1 < k {
      0
    } else {
      p - k + 1
    };

    let m = n / b;
    let v = m * v + pv;
    result += v;
  }
  
  if k == 0 {
    result -= n;
  }
  
  println!("{}", result);
}
