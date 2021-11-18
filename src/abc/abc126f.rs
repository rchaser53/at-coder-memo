/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::Reverse;
use std::cmp::Ordering;

fn main() {
  input! {
    m:usize,
    k:usize
  }

  if 2usize.pow(m as u32) <= k {
    println!("-1");
    return
  }

  if m == 0 {
    println!("0 0");
    return
  } else if m == 1 {
    if k == 1 {
      println!("-1");
    } else {
      println!("0 1 1 0");
    }
    return
  }

  let mut result = vec![];
  for i in 0..2usize.pow(m as u32) {
    if i != k {
      result.push(i);
    }
  }
  let mut cloned = result.clone();
  cloned.reverse();
  result.push(k);
  for v in cloned {
    result.push(v);
  }
  result.push(k);
  println!("{}", result.into_iter().map(|v| v.to_string()).collect::<Vec<String>>().join(" "));
}