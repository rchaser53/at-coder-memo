/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::*;

fn main() {
  input! {
    l:usize,
    r:usize
  }
  
  let mut result = 0;
  let mut memo = vec![0;r+1];
  let l = std::cmp::max(2, l);
  for i in (2..=r).rev() {
    let x = r / i - (l-1) / i;
    memo[i] = x * x;
    for j in ((2*i)..=r).step_by(i) {
      memo[i] -= memo[j];
    }
    result += memo[i];

    if l <= i {
      result -= 2 * x - 1;
    }
  }
  println!("{}", result);
}