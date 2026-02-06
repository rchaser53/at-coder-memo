/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::marker::PhantomData;
use std::cmp::*;
use std::collections::*;

fn main() {
  input! {
    n:usize,
    a:[usize;n]
  }

  let mut stack = vec![];
  for v in a {
    stack.push(v);
    while stack.len() >= 4 {
      let len = stack.len();
      if stack[len-1] == stack[len-2] &&
      stack[len-2] == stack[len-3] &&
      stack[len-3] == stack[len-4] {
        for _ in 0..4 {
          stack.pop();
        }
      } else {
        break;
      }
    }
  }
  println!("{}", stack.len());
}