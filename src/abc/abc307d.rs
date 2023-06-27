/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::collections::*;

fn main() {
  input! {
    n:usize,
    s:Chars
  }

  let mut result = vec![];
  let mut memos = vec![];
  for c in s {
    if c == '(' {
      memos.push(vec!['(']);
    } else if c == ')' {
      if memos.is_empty() {
        result.push(')');
      } else {
        memos.pop();
      }
    } else {
      if memos.is_empty() {
        result.push(c);
      } else {
        let li = memos.len()-1;
        memos[li].push(c);
      }
    }
  }

  for memo in memos {
    for c in memo {
      result.push(c);
    }
  }
  println!("{}", result.into_iter().map(|v|v.to_string()).collect::<String>());
}