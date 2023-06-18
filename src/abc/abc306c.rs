/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::collections::*;

fn main() {
  input! {
    n:usize,
    a:[Usize1;3*n]
  }

  let mut memo = vec![0;n];
  let mut stack = vec![];

  for v in a {
    memo[v] += 1;
    if memo[v] == 2 {
      stack.push(v+1);
    }
  }

  println!("{}", stack.into_iter().map(|v| v.to_string()).collect::<Vec<String>>().join(" "));
}