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
    s:Chars
  }

  let n = s.len();
  let mut result = 0usize;

  for i in 0..n {
    let ln = i;
    let rn = n - i - 1;
    if s[i] == 'U' {
      result += ln * 2 + rn;
    } else {
      result += rn * 2 + ln;
    }
  }
  println!("{}", result);
}
