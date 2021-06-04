/* OUTPUT FILE */
#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;
use std::collections::*;

pub fn main(
) {
  input! {
    n:usize,
    m:usize,
    vals:[usize;n]
  }

  let mut count = 0;
  let limit = vals.iter().sum::<usize>();
  for v in vals {
    if 4 * m * v >= limit {
      count += 1;
    }
  }

  if m <= count {
    println!("Yes");
  } else {
    println!("No");
  }
}
