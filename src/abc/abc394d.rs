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
      if (stack[len-1] == '<' && c == '>') ||
        (stack[len-1] == '[' && c == ']') ||
        (stack[len-1] == '(' && c == ')') {
        stack.pop();
        
        if let Some(cc) = stack.pop() {
          s.push_front(cc);
        }
      } else {
        stack.push(c);
      }
    } else {
      stack.push(c);
    }
  }

  if stack.is_empty() {
    println!("Yes");
  } else {
    println!("No");
  }
}