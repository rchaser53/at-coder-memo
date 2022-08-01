/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use std::collections::*;
use std::cmp::Reverse;
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;

fn main() {
  input! {
    n:usize,
    vals:[Usize1;n]
  }
  if n == 1 {
    println!("0");
    return
  }

  let mut result = 0;
  let mut count = 0usize;
  if vals[n-1] == n-1 {
    count += 1;
  }

  for i in (0..n-1).rev() {
    if i == vals[i] {
      result += count;
      count += 1;
    }
  }

  for i in 0..n {
    let v = vals[i];
    if v != i && i < v {
      if vals[v] == i {
        result += 1;
      }
    }
  }
  println!("{}", result);
}