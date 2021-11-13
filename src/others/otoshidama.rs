#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;
use std::collections::VecDeque;

fn main() {
  input!{
    n: usize,
    y: usize
  }
  
  for i in 0..=n {
    let v = i * 1000;
    for ii in 0..=n-i {
      let vv = ii * 5000;
      if y < v + vv { break }
      let left = y - v - vv;
      if left % 10000 == 0 && (left / 10000) + i + ii == n {
        println!("{} {} {}", n - i - ii, ii, i);
        return
      }
    }
  }
  println!("-1 -1 -1");
}
