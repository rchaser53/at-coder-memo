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
    a: isize,
    b: isize,
    c: isize
  }

  let ab = b - a;
  let bc = c - b;
  if bc <= ab {
    println!("{}", ab - bc);
  } else {
    let diff = bc - ab;
    if diff % 2 == 0 {
      println!("{}", diff / 2);
    } else {
      println!("{}", diff / 2 + 2);
    }
  }
}
