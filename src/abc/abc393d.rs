/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use std::cmp::Reverse;
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::collections::*;

fn helper(s: &Vec<char>, ci:usize) -> usize {
  let n = s.len();

  let mut result = 0;
  let mut count = 0;
  for i in (0..ci).rev() {
    if s[i] == '1' {
      result += count;
    } else {
      count += 1;
    }
  }

  count = 0;
  for i in ci+1..n {
    if s[i] == '1' {
      result += count;
    } else {
      count += 1;
    }
  }

  result
}

fn main() {
  input! {
    n:usize,
    s:Chars
  }

  let mut memo = vec![];
  for i in 0..n {
    if s[i] == '1' {
      memo.push(i);
    }
  }

  let hi = memo.len() / 2;
  if memo.len() % 2 == 0 {
    println!("{}", helper(&s, memo[hi-1]).min(helper(&s, memo[hi])));
  } else {
    println!("{}", helper(&s, memo[hi]));
  }
}