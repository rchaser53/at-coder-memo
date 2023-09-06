/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::*;
use std::collections::*;

fn main() {
  input! {
    n:usize,
    d:usize,
    p:usize,
    mut f:[usize;n],
  }
  let mut result = f.iter().sum::<usize>();
  let mut sum = result;
  f.sort();

  let mut num = 1;
  while !f.is_empty() {
    let temp = num * p;
    let mut i = 0;
    while i < d && !f.is_empty() {
      sum -= f.pop().unwrap();
      i += 1;
    }

    result = result.min(temp + sum);
    num += 1; 
  }

  println!("{}", result);
}