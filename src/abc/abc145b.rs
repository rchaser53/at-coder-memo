/* OUTPUT FILE */
#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::Reverse;

pub fn main(
) {
  input! {
    n:usize,
    s:Chars,
  }

  if n % 2 == 1 {
    println!("No");
    return
  }

  let half = n / 2;
  for i in 0..half {
    if s[i] != s[half+i] {
      println!("No");
      return
    }
  }
  println!("Yes");
}
