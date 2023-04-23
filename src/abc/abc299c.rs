/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::*;

fn main() {
  input!{
    n:usize,
    s:Chars
  }
    
  let mut memo = vec![];
  if s[0] == '-' {
    memo.push(-1);
  } else {
    memo.push(1);
  }

  for i in 1..n {
    let li = memo.len() - 1;
    if s[i] == '-' {
      if memo[li] != -1 {
        memo.push(-1);
      }
    } else {
      if memo[li] == -1 {
        memo.push(1);
      } else {
        memo[li] += 1;
      }
    }
  }

  let m = memo.len();
  let mut result = -1;
  for i in 1..m {
    if memo[i-1] == -1 && memo[i] != -1 {
      result = result.max(memo[i]);
    }
  }
  for i in (0..m-1).rev() {
    if memo[i+1] == -1 && memo[i] != -1 {
      result = result.max(memo[i]);
    }
  }

  println!("{}", result);
}