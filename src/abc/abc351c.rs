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
  for v1 in a {
    stack.push(v1);
    while stack.len() > 1 {
      let v1 = stack.pop().unwrap();
      let v2 = stack.pop().unwrap();
      if v1 == v2 {
        stack.push(v1+1);
      } else {
        stack.push(v2);
        stack.push(v1);
        break
      }
    }
  }
  println!("{}", stack.len());
}