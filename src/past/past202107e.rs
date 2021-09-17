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

  for i in 1..=30 {
    let mut val = 1;
    for j in 1..=30 {
      val *= 3;
      if i == j {
        val += 1;
      }
    }

    if val == n {
      println!("{}", i);
      return
    }
  } 

  println!("-1");
}