#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;

#[fastout]
fn main() {
  input!{
    s: Chars
  }
  let mut memo:Vec<isize> = vec![0];
  memo[0] = if s[0] == 'W' {
    1
  } else {
    -1
  };
  
  for i in 1..s.len() {
    let last_index = memo.len() - 1;
    let c = s[i];
    let last = memo[last_index];
    if c == 'W' {
      if 0 < last {
        memo[last_index] += 1; 
      } else {
        memo.push(1);
      }
    } else {
      if last < 0 {
        memo[last_index] -= 1;
      } else {
        memo.push(-1);
      }
    }
  }
  println!("{}", memo.len()-1);
}