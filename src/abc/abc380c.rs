/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::*;

fn main() {
  input! {
    n:usize,
    k:usize,
    s:Chars
  }

  let mut stack = vec![];
  
  if s[0] == '1' {
    stack.push(('1',1));
  } else {
    stack.push(('0',1));
  }

  for i in 1..n {
    let li = stack.len() - 1;
    let lc = stack[li].0;
    let c = s[i];
    if c == '1' {
      if c == lc {
        stack[li].1 += 1;
      } else {
        stack.push(('1',1));
      }
    } else {
      if c == lc {
        stack[li].1 += 1;
      } else {
        stack.push(('0',1));
      }
    }
  }

  let mut count = 0;
  for i in 0..stack.len() {
    if stack[i].0 == '1' {
      count += 1;
    }

    if count == k {
      stack.swap(i-1,i);
      break
    }
  }

  let mut result = vec!['0';n];
  let mut i = 0;
  for (c,num) in stack {
    for _ in 0..num {
      result[i] = c;
      i += 1;
    }
  }  
  println!("{}", result.into_iter().map(|v| v.to_string()).collect::<String>());
}