#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;
use std::collections::{HashMap, VecDeque};

fn main() {
  input!{
    mut s: Chars,
    mut t: Chars
  }
  
  s.sort();
  t.sort();
  t.reverse();
  let s: String = s.into_iter().map(|v| v.to_string()).collect();
  let t: String = t.into_iter().map(|v| v.to_string()).collect();

  if s < t {
    println!("Yes");
  } else {
    println!("No");
  }
}
