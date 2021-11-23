/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::Reverse;
use std::cmp::Ordering;

fn main() {
  input! {
    mut n:usize,
    l:usize,
    r:usize
  }

  let limit = 61;
  let mut temp = vec![false;limit];
  while 0 < n {
    let mut i = 0;
    while (1 << i) <= n {
      i += 1;
    }
    n -= 1 << (i-1);
    temp[i-1] = true;
  }

  let mut result = 0usize;
  for i in 0..limit {
    let v = temp[i];
    if !v { continue }
    let right = std::cmp::min(r, (1<<(i+1)) - 1);
    let left = std::cmp::max(l, 1 << i);


    if right < left { continue }
    result += right - left + 1;
  }
  println!("{}", result)
}