/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use std::cmp::Reverse;
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::collections::*;

fn main() {
  input! {
    n:usize,
    m:usize,
    a:[Chars;n],
    b:[Chars;m]
  }

  for i in 0..n-m+1 {
    for j in 0..n-m+1 {
      let mut is_match = true;
      for i2 in 0..m {
        for j2 in 0..m {
          if a[i+i2][j+j2] != b[i2][j2] {
            is_match = false;
            break;
          }
        }
      }
      if is_match {
        println!("{} {}", i+1, j+1);
        return;
      }
    }
  }
}