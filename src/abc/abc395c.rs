/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::Reverse;
use std::collections::*;

fn main() {
  input!{
    n:usize,
    a:[usize;n]
  }
  let mut map = HashMap::new();
  for i in 0..n {
    map.entry(a[i]).or_insert(vec![]).push(i);
  }

  let default = 10usize.pow(10);
  let mut result = default;
  for (_, arr) in map {
    if arr.len() > 1 {
      let m = arr.len();
      for i in 0..m-1 {
        if arr[i+1]- arr[i] < result {
          result = arr[i+1] - arr[i] + 1;
        }
      }
    }
  }

  if result == default {
    println!("-1");
  } else {
    println!("{}", result);
  }
}