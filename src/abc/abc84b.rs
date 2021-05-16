#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;
use std::collections::VecDeque;

fn main() {
  input!{
    a:usize,
    b:usize,
    s:Chars
  }
  
  if s[a] != '-' {
    println!("No");
    return
  }
  
  for i in 0..=a+b {
    if i == a { continue }
    if s[i] < '0' || '9' < s[i] {
      println!("No");
      return
    }
  }
  println!("Yes");
}