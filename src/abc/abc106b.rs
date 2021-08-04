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
  }

  let mut count = 0;
  for i in 1..=n {
    if i % 2 == 0 { continue }
    let mut temp = 0;
    for j in 1..=i {
      if i % j == 0 { 
        temp += 1;
      }
    }

    if temp == 8 {
      count += 1;
    }
  }
  println!("{}", count);
}
