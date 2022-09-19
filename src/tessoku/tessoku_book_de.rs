/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use std::collections::*;
use std::cmp::Reverse;
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;

fn main() {
  input! { 
    n:usize,
    k:usize,
    a:[usize;k]
  }
  let mut memo = vec![false;n+1];
  for i in 0..n {
    if !memo[i] {
      for j in &a {
        let ni = i + j;
        if n < ni { continue }
        memo[ni] = true;
      }
    }
  }

  if memo[n] {
    println!("First");
  } else {
    println!("Second");
  }
}