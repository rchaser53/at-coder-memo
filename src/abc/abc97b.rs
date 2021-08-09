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
    x:usize
  }

  let mut result = 1;
  for i in 2..=x {
    for j in 2.. {
      let v = i.pow(j as u32);
      if v <= x {
        result = std::cmp::max(result, v);
      } else {
        break
      }
    }
  }
  println!("{}", result);
}
