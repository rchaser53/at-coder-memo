/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::Reverse;

fn main() {
  input! {
    n:usize,
    vals:[isize;n]
  }
  
  let mut now = 0;
  let mut temp = 0;
  let mut temp_max = 0;
  let mut result = 0;
  for v in vals {
    result = std::cmp::max(result, now + temp_max);
    temp += v;
    temp_max = std::cmp::max(temp_max, temp);
    now += temp;
  }
  result = std::cmp::max(now, result);

  println!("{}", result);
}