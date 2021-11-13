#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;
use std::collections::*;

fn main() {
  input! {
    n: usize,
    m: usize,
    base: [Chars;n],
    inside: [Chars;m]
  }

  for r in 0..=n-m {
    for c in 0..=n-m {
      let mut success = true;
      for i in 0..m {
        for ii in 0..m {
          if inside[i][ii] != base[r+i][c+ii] {
            success = false;
            break
          }
        }
        if !success { break }
      }
      if success {
        println!("Yes");
        return
      }
    }
  }
  println!("No");
}