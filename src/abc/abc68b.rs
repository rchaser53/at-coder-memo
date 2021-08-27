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
    n:usize
  }

  let mut result = 1;
  let mut max = 0;
  for bi in 1..=n {
    let mut i = bi;
    let mut count = 0;
    while i % 2 == 0 {
      i /= 2;
      count += 1;
    }
    if max < count {
      max = count;
      result = bi;
    }
  }
  println!("{}", result);
}
