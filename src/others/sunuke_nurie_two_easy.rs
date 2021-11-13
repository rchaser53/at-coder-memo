#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;

#[fastout]
fn main() {
  input!{
    w: usize,
    h: usize,
    n: usize,
    vals: [(usize, usize, usize);n]
  }

  let mut width = (0, w);
  let mut height = (0, h);
  for (x, y, a) in vals {
    match a {
      1 => {
        if x < width.0 { continue }
        width = (x, width.1);
      },
      2 => {
        if width.1 < x { continue }
        width = (width.0, x);
      },
      3 => {
        if y < height.0 { continue }
        height = (y, height.1);
      },
      4 => {
        if height.1 < y { continue }
        height = (height.0, y);
      },
      _ => unreachable!()
    }
  }
  
  if width.1 <= width.0 || height.1 <= height.0 {
    println!("0");
  } else {
    println!("{}", (width.1 - width.0) * (height.1 - height.0));
  }
}