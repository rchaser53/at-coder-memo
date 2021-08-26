/* OUTPUT FILE */
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_assignments)]
use proconio::input;
use proconio::marker::*;
use std::collections::*;

pub fn main(
) {
  input! {
    n:usize,
    s:Chars,
    t:Chars
  }

  let mut min = 0;
  for i in 0..n {
    let mut success = true;
    for j in 0..=i {
      if s[n-i-1+j] != t[j] {
        success = false;
      }
    }
    if success {
      min = i + 1;
    }
  }

  println!("{}", 2 * n - min);
}
