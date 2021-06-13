/* OUTPUT FILE */
use proconio::input;
use proconio::marker::*;
use std::collections::*;

pub fn main(
) {
  input! {
    mut vals:[isize;4],
  }

  loop {
    vals[2] -= vals[1];
    if vals[2] <= 0 {
      println!("Yes");
      return
    }

    vals[0] -= vals[3];
    if vals[0] <= 0 {
      println!("No");
      return
    }
  }
}