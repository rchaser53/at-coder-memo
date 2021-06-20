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
    n:usize    
  }

  let mut count = 0;
  let mut total = 0;
  while total < n {
    count += 1;
    total += count;
  }
  println!("{}", count);
}