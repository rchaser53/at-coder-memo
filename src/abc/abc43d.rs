/* OUTPUT FILE */
#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;
use std::collections::*;

pub fn main(
) {
  input! {
    s:Chars
  }

  let len = s.len();
  for i in 0..26 {
    let target = ('a' as u8 + i) as char;
    for j in 0..len-1 {
      if s[j] == target && s[j+1] == target {
        println!("{} {}", j+1, j+2);
        return
      }

      if j + 2 < len && s[j] == target && s[j+2] == target {
        println!("{} {}", j+1, j+3);
        return
      }
    }
  }
  println!("-1 -1");
}
