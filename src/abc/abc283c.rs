/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::collections::*;

fn main() {
  input! {
    s:Chars
  }

  let mut count = 0;
  let n = s.len();

  let mut i = 0;
  while i < n-1 {
    if s[i] == '0' && s[i+1] == '0' {
      i += 1;
    }
    count += 1;
    i += 1;
  }

  if i == n-1 {
    count += 1;
  }

  println!("{}", count);
}