/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::*;

fn main() {
  input! {
    n:usize,
    a:[Usize1;n],
    w:[usize;n]
  }

  let mut map = HashMap::new();
  for i in 0..n {
    let entry = map.entry(a[i]).or_insert(vec![]);
    entry.push(w[i]);
  }

  let mut result = 0;
  for (_, mut arr) in map {
    arr.sort();
    for i in 0..arr.len()-1 {
      result += arr[i];
    }
  }
  println!("{}", result);
}