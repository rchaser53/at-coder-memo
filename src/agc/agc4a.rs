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
    vals:[usize;3]
  }
  
  let mut result = usize::max_value();
  for i in 0..3 {
    let v = vals[i];
    if v % 2 == 0 {
      result = 0;
    } else {
      let mut temp = 1;
      for j in 0..3 {
        if i == j { continue }
        temp *= vals[j];
      }
      result = std::cmp::min(result, temp);
    }
  }
  println!("{}", result);
}
