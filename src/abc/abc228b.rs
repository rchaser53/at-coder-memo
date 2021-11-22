/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::Reverse;
use std::cmp::Ordering;

fn main() {
  input! {
    n:usize,
    x:Usize1,
    vals:[Usize1;n]
  }

  let mut result = vec![false;n];
  result[x] = true;
  let mut ni = vals[x];
  while !result[ni] {
    result[ni] = true;
    ni = vals[ni];
  }
  let mut output = 0;
  for v in result {
    if v {
      output += 1;
    }
  }
  println!("{}", output);
}