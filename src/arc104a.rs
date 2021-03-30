#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;
use std::collections::VecDeque;

fn main() {
  input!{
    a:isize,
    b:isize,
  }
  
  for x in -200..=200 {
    for y in -200..=200 {
      if x + y == a && x - y == b {
        println!("{} {}", x, y);
        return
      }
    } 
  }
}