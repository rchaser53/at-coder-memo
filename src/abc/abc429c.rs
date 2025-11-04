/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use std::cmp::Reverse;
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::collections::*;

fn main() {
  input! {
    n:usize,
    a:[usize;n]
  }
  
  let mut map = HashMap::new();
  for v in a {
    *map.entry(v).or_insert(0) += 1;
  }
  let mut result = 0usize;
  for (_, num) in map {
    let left = n - num;
    let x = num * (num-1) / 2;
    result += left * x;
  }
  println!("{}", result);
}