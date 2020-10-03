#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;
use std::collections::VecDeque;
 
fn main() {
  input!{
    n: usize,
    m: usize
  }
  
  println!("{}", 2usize.pow(m as u32) * (1900 * m + 100 * (n - m)));
}