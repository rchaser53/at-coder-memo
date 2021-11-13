#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;
use std::collections::VecDeque;

fn main() {
  input!{
    a: usize,
    b: usize
  }
  
  let mut result = 0;
  for i in a..=b {
    let chars = i.to_string().chars().collect::<Vec<char>>();
    if chars[0] == chars[4] && chars[1] == chars[3] {
      result += 1;
    }
  }
 
  println!("{}", result);
}