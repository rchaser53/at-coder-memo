use proconio::input;
use proconio::marker::*;
use std::collections::*;

fn main() {
  input!{
    s:Chars
  }

  let a = s.len() % 2 == 0;
  let b = s[0] == s[s.len()-1];
  if a ^ b {
    println!("Second");
  } else {
    println!("First");
  }
}