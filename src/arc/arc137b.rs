/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::*;

fn main() {
  input! {
    n: usize,
    vals:[i32;n]
  }
  
  let mut now = 0;
  let mut min = 0;
  let mut max = 0;
  let mut zero = 0;
  let mut one = 0;

  for i in 0..n {
    if vals[i] == 1 {
      now += 1;
    } else {
      now -= 1;
    }

    zero = zero.min(now - max);
    one = one.max(now - min);
    min = min.min(now);
    max = max.max(now);
  }

  println!("{}", one - zero + 1);
}