/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::collections::*;

fn main() {
  input! {
    n:usize,
    s:Chars
  }

  let mut que = VecDeque::new();
  que.push_back(format!("{}", n));
  for i in (0..n).rev() {
    if s[i] == 'L' {
      que.push_back(format!("{}", i));
    } else {
      que.push_front(format!("{}", i));
    }
  }

  println!("{}", que.into_iter().collect::<Vec<String>>().join(" "));
}