https://atcoder.jp/contests/arc130/tasks/arc130_a
/* OUTPUT FILE */
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_assignments)]

use proconio::input;
use proconio::marker::*;
use std::collections::*;

pub fn main(
) {
  input! {
    n:usize,
    s:Chars
  }

  let mut memo = vec![];
  let mut result = 0;
  for c in s {
    if memo.is_empty() {
      memo.push(c);  
    } else {
      if memo[0] != c {
        memo = vec![c];
      } else {
        memo.push(c);
        result += memo.len() - 1;
      }
    }
  }
  println!("{}", result);
}
