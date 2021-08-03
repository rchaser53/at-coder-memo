/* OUTPUT FILE */
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_assignments)]
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::Reverse;

pub fn main(
) {
  input! {
    n:usize,
    m:usize,
    x:isize,
    y:isize,
    xs:[isize;n],
    ys:[isize;m]
  }

  for z in x+1..=y {
    let mut success = true;
    for &v in &xs {
      if z <= v {
        success = false;
      }
    }

    for &v in &ys {
      if v < z {
        success = false;
      }
    }

    if success {
      println!("No War");
      return
    }
  }
  println!("War");
}