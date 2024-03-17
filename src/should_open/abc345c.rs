/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::*;

struct Solution {}
fn main() {
  input! {
    s:Chars
  }

  let mut memo = vec![0usize;26];
  let n = s.len();
  for i in 0..n {
    memo[s[i] as usize - 'a' as usize] += 1;
  }

  let mut result = 0usize;
  for i in 0..26 {
    if memo[i] > 1 {
      result += 1;
      break
    }
  }

  for i in 0..n-1 {
    let ci = s[i] as usize - 'a' as usize;
    memo[ci] -= 1;
    for j in 0..26 {
      if j == ci { continue }
      result += memo[j];
    }
  }
  println!("{}", result);
}