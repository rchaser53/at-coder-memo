/* OUTPUT FILE */
#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::Reverse;

const MOD:usize = 998244353;
pub fn main(
) {
  input! {
    h:usize,
    w:usize,
    rows:[Chars;h]
  }

  let mut base = 1usize;
  for i in 0..w {
    let mut red = 0usize;
    let mut blue = 0usize;
    let mut unknown = 0usize;
    for j in 0..=i {
      if h <= j { break }
      if rows[j][i-j] == 'R' {
        red += 1;
      } else if rows[j][i-j] == 'B' {
        blue += 1;
      } else {
        unknown += 1;
      }
    }
    if 0 < red && 0 < blue {
      println!("0");
      return
    } else if 0 < unknown {
      if red == 0 && blue == 0 {
        base *= 2;
        base %= MOD;
      }
    }
  }

  for i in 1..h {
    let mut red = 0usize;
    let mut blue = 0usize;
    let mut unknown = 0usize;
    for j in 0..w {
      let c = w - j - 1;
      let r = i + j;
      if h <= r { break }
      if rows[r][c] == 'R' {
        red += 1;
      } else if rows[r][c] == 'B' {
        blue += 1;
      } else {
        unknown += 1;
      }
    }
    if 0 < red && 0 < blue {
      println!("0");
      return
    } else if 0 < unknown {
      if red == 0 && blue == 0 {
        base *= 2;
        base %= MOD;
      }
    }
  }
  println!("{}", base);
}
