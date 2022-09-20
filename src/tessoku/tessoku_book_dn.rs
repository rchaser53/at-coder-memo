/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use std::collections::*;
use std::cmp::Reverse;
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;

fn main() {
  input! { 
    mut x:usize,
    mut y:usize
  }

  let mut result = vec![];
  while x != 1 || y != 1 {
    result.push((x,y));
    if x > y {
      x -= y;
    } else {
      y -= x;
    }
  }

  println!("{}", result.len());
  for (a, b) in result.iter().rev() {
    println!("{} {}", a, b);
  }
}