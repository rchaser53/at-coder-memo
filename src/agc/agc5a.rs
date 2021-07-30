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

  let mut stack = vec![];
  for c in s {
    if stack.is_empty() {
      stack.push(c);
    } else if stack.last().unwrap() == &'S' && c == 'T' {
      stack.pop();
    } else {
      stack.push(c);
    }
  }
  println!("{}", stack.len());
}