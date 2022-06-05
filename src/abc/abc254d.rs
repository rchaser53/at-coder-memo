/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::*;

fn main() {
  input! {
    n:usize,
  }
  
  let mut result = 0;
  for i in 1..=n {
    let mut k = i;
    for d in 2.. {
      if k < d * d { break }
      while k % (d * d) == 0 {
        k /= d * d;
      }
    }
    for d in 1.. {
      if n < k * d * d { break }
      result += 1;
    }
  }
  println!("{}", result);
}