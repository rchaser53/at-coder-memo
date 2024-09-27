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
    m:usize,
    s:Chars
  }

  let mut result = 0;
  let mut normal = m;
  let mut color = 0;
  for c in s {
    if c == '0' {
      result = result.max(color);
      normal = m;
      color = 0;
    } else if c == '1' {
      if 0 < normal {
        normal -= 1;
      } else {
        color += 1;
      }
    } else {
      color += 1;
    }
  }

  result = result.max(color);

  println!("{}", result);
}