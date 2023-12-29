/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::*;

fn main() {
  input! {
    _n:usize,
    s:Chars
  }

  let mut result = 0usize;
  let mut zero = 0;
  let mut one = 0;
  for c in s {
    (one, zero) =
    if c == '0' {
      (one+zero, 1)
    }
    // 1
    else {
      (zero + 1, one)
    };
    result += one;
  }
  
  println!("{}", result); 
}