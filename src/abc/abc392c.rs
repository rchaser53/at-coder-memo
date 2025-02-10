/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use std::cmp::Reverse;
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::collections::*;

fn main() {
  input! {
    n:usize,
    p:[Usize1;n],
    q:[Usize1;n]
  }
  
  let mut result = vec![0;n];
  let mut memo = vec![0;n];
  for i in 0..n {
    // ゼッケンから人 q[i] == i
    memo[q[i]] = i;
  }

  for i in 0..n {
    result[i] = q[p[memo[i]]] + 1;
  }

  println!("{}", result.into_iter().map(|v| v.to_string()).collect::<Vec<String>>().join(" "));
}