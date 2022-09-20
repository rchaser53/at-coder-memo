/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use std::collections::*;
use std::cmp::Reverse;
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;

fn main() {
  input! { 
    n:usize,
    d:usize,
    vals:[(Usize1,usize);n]
  }

  let mut memo = vec![vec![];d];
  for (sd, v) in vals {
    memo[sd].push(v);
  } 

  let mut heap = BinaryHeap::new();
  let mut result = 0;
  for i in 0..d {
    for v in &memo[i] {
      heap.push(v);
    }
    if let Some(v) = heap.pop() {
      result += v;
    }
  }

  println!("{}", result);
}