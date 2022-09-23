/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use std::collections::*;
use std::cmp::Reverse;
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;

fn main() {
  input! {
    n:Usize1,
  }

  let mut memo = vec![0;1024];
  for i in 0..1024 {
    let mut temp = 0usize;
    for j in 0..10 {
      if i >> j & 1 == 1 {
        temp += 10usize.pow(j) * 4;
      } else {
        temp += 10usize.pow(j) * 7;
      }
    }
    memo[i as usize] = temp;
  }
  memo.sort();
  println!("{}", memo[n]);
}