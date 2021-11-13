#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;
use std::collections::VecDeque;

fn main() {
  input!{
    n: usize,
    m: usize
  }  
  if n == 1 && m == 1 {
    println!("1");
  } else if n > 1 && m == 1 {
    println!("{}", n - 2);
  } else if m > 1 && n == 1 {
    println!("{}", m - 2);
  } else {
    println!("{}", (n-2)*(m-2));
  }
}
