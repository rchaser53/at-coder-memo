/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::*;

fn main() {
  input! {
    n:usize,
    vals:[(usize,usize);n]
  }

  let mut result = 0;  
  for (a,b) in vals {
    let diff = b - a;
    if 5 <= diff % 10 {
      result += 1;
    }
    if 50 <= diff % 100 {
      result += 1;
    }
  }
  println!("{}", result);
}