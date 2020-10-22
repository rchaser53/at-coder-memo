#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;

#[fastout]
fn main() {
  input! {
    s: Chars
  }
  
  let mut stack = vec![];
  for c in s {
    if c == 'B' {
      if !stack.is_empty() {
        stack.pop();
      }
    } else {
      stack.push(c);
    }
  }
  println!("{}", stack
    .into_iter()
    .map(|c| c.to_string())
    .collect::<String>()
  );
}