use proconio::input;
use proconio::marker::*;
use std::collections::*;

fn main() {
  input!{
    mut s:Chars
  }

  let mut result = 0usize;
  let mut i = 0;
  while i < s.len() {
    if i == s.len() - 1 {
      result += 1;
      break
    }

    if s[i] == s[i+1] {
      if i+1 == s.len() - 1 {
        result += 1;
        break
      } else {
        i += 3;
        result += 2;
      }
    } else {
      result += 1;
      i += 1;
    }
  }
  
  println!("{}", result);
}