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
    m:usize,
    c:isize,
    bases:[isize;m],
    vals:[[isize;m];n]
  }

  let mut result = 0;
  for i in 0..n {
    let mut temp = c;
    for j in 0..m {
      temp += vals[i][j] * bases[j];
    }
    if 0 < temp {
      result += 1;
    }
  }
  println!("{}", result);
}
