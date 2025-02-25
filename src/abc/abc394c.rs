/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use std::cmp::Reverse;
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::collections::*;

fn main() {
  input! {
    s:Chars
  }

  let mut s = s.into_iter().collect::<VecDeque<char>>();
  let mut stack = vec![];
  while let Some(c) = s.pop_front()  {
    let len = stack.len();
    if len > 0 {
      if stack[len-1] == 'W' && c == 'A' {
        stack.pop();
        s.push_front('C');
        s.push_front('A');
      } else {
        stack.push(c);
      }
    } else {
      stack.push(c);
    }
  }

  println!("{}", stack.into_iter().collect::<String>());
}