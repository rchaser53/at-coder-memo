/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::Reverse;

fn main() {
  input! {
    n:usize,
    a:usize,
    b:usize,
    s:Chars
  }

  let mut result = vec![false;n];
  let limit = a + b;
  let mut count = 0;
  let mut foreign = 0;
  for i in 0..n {
    let c = s[i];
    let is_under = count < limit;
    if c == 'a' && is_under {
      count += 1;
      result[i] = true;
    } else if c == 'b' && is_under {
      foreign += 1;
      if foreign <= b {
        count += 1;
        result[i] = true;
      }
    }
  }

  for v in result {
    if v {
      println!("Yes");
    } else {
      println!("No");
    }
  }
}