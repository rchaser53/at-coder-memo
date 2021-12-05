/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::Reverse;

fn main() {
  input! {
    s:Chars
  }

  let mut left = 0;
  let mut right = s.len() - 1;
  let mut result = 0usize;
  while left < right {
    if s[left] == 'x' && s[right] != 'x' {
      result += 1;
      left += 1;
    } else if s[left] != 'x' && s[right] == 'x' {
      result += 1;
      right -= 1;
    } else if s[left] != s[right] {
      println!("-1");
      return
    } else {
      left += 1;
      right -= 1;
    }
  }
  
  println!("{}", result);
}