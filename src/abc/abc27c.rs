/* OUTPUT FILE */
use proconio::input;
use proconio::marker::*;
use std::collections::*;

pub fn main(
) {
  input! {
    n:usize,
  }
  let mut x = n + 1;
  let mut flag = true;
  while 1 < x {
    if flag {
      if x % 2 == 1 {
        x += 1;
      }
      x /= 2;
    } else {
      x /= 2;
    }
    flag = !flag;
  }
  
  if flag {
    println!("Takahashi");
  } else {
    println!("Aoki");
  }
}
