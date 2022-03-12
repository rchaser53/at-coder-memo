/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::Ordering::*;

fn main() {
  input! {
    s:Chars
  }

  let mut memo = vec![0;3];
  for c in s {
    let i = (c as u8 - 'a' as u8) as usize;
    memo[i] += 1i32;
  }

  if (memo[0]-memo[1]).abs() <= 1
    && (memo[1]-memo[2]).abs() <= 1
    && (memo[0]-memo[2]).abs() <= 1 {
      println!("YES");
    } else {
      println!("NO");
    }
}