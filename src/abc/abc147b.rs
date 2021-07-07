/* OUTPUT FILE */
use proconio::input;
use proconio::marker::*;
use std::collections::*;

pub fn main(
) {
  input! {
    mut s: Chars
  }

  let n = s.len();
  let mut c = 0;
  for i in 0..n {
    if s[i] != s[n-1-i] {
      c += 1;
      s[i] = s[n-1-i];
    }
  }
  
  println!("{}", c);
}
