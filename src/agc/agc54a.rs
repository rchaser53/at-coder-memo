use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::Reverse;

fn main() {
  input! {
    n:usize,
    s:Chars
  }

  if s[0] != s[n-1] {
    println!("1");
    return
  }

  for i in 1..n-1 {
    if s[i] != s[0] && s[i+1] != s[0] {
      println!("2");
      return
    }
  }
  println!("-1");
}