/* OUTPUT FILE */
#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::Reverse;

pub fn main(
) {
  input! {
    h:usize,
    w:usize,
    mut a: [[isize;w];h],
    b:[[isize;w];h]
  }

  let mut count = 0;
  for i in 0..h-1 {
    for j in 0..w-1 {
      if a[i][j] < b[i][j] {
        let diff = b[i][j] - a[i][j];
        a[i][j] += diff;
        a[i+1][j] += diff;
        a[i][j+1] += diff;
        a[i+1][j+1] += diff;
        count += diff;
      } else if b[i][j] < a[i][j] {
        let diff = a[i][j] - b[i][j];
        a[i][j] -= diff;
        a[i+1][j] -= diff;
        a[i][j+1] -= diff;
        a[i+1][j+1] -= diff;
        count += diff;
      }
    }
  }

  for i in 0..h {
    for j in 0..w {
      if a[i][j] != b[i][j] {
        println!("No");
        return
      }
    }
  }
  println!("Yes");
  println!("{}", count);
}
