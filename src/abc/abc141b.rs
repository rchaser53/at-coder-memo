/* OUTPUT FILE */
use proconio::input;
use proconio::marker::*;
use std::collections::*;

pub fn main(
) {
  input! {
    s:Chars,
  }

  for i in 0..s.len() {
    if i % 2 == 0 && s[i] == 'L' {
      println!("No");
      return
    } else if i % 2 == 1 && s[i] == 'R' {
      println!("No");
      return
    }
  }
  println!("Yes");
}
