/* OUTPUT FILE */
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_assignments)]

use proconio::{input};
use proconio::marker::*;
use std::collections::*;

pub fn main(
) {
  input! {
    n:usize,
    s:Chars,
  }
  
  let mut count = 0;
  let mut last = 'z';
  for i in 0..n {
    let c = s[i];
    if last == 'z' {
      if c == 'A' {
        last = 'A';
      }
    } else if last == 'A' {
      if c == 'B' {
        last = 'B';
      } else if c == 'A' {
        last = 'A';
      } else {
        last = 'z';
      }
    } else if last == 'B' {
      if c == 'C' {
        count += 1;
      }
      last = 'z';
    }
  }
  println!("{}", count);
}