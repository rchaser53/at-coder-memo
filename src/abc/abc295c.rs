/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::Reverse;
use std::cmp::Ordering;
use std::collections::*;

fn main() {
  input! {
    n:usize,
    a:[i32;n]
  }
  
  let mut map = HashMap::new();
  for v in a {
    *map.entry(v).or_insert(0) += 1;
  }

  let mut result = 0;
  for (_, num) in map {
    result += num / 2;
  }
  println!("{}", result);
}