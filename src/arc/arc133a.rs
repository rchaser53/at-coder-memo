/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::*;

fn main() {
  input! {
    n:usize,
    vals:[usize;n]
  }

  let mut target = 0;
  for &v in &vals {
    if target > v {
      break
    }
    target = v;
  }

  let mut result = vec![];
  for v in vals {
    if target != v {
      result.push(v);
    }
  }

  println!("{}", result.into_iter().map(|v| v.to_string()).collect::<Vec<String>>().join(" "));
}