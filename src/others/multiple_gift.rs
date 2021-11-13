#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;
use std::collections::VecDeque;

fn main() {
  input!{
    mut x: usize,
    y: usize
  }
  
  let mut count = 1;
  while 2 * x <= y {
    count += 1;
    x *= 2;
  }
  println!("{}", count);
}
