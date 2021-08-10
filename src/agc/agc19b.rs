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

  let mut memo = vec![0;26];
  for c in s {
    memo[(c as u8 - 'a' as u8) as usize] += 1;
  }
  let mut result = 0usize;
  for i in 0..26 {
    for j in i+1..26 {
      result += memo[i] * memo[j];
    }
  }
  println!("{}", result + 1);
}
