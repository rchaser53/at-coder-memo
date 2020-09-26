#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;
use std::collections::HashSet;

fn main() {
  input! {
    a: usize,
    b: usize,
    c: usize,
  }
  
  let mut memo = vec![a,b,c];
  memo.sort();
  let a = memo[0];
  let b = memo[1];
  let c = memo[2];
  
  let cb = (c - b) % 2;
  let ca = (c - a) % 2;
  
  if cb == 0 && ca == 0 {
    println!("{}", (c - b) / 2 + (c - a) / 2);
  } else if (cb == 1 && ca == 0) || cb == 0 && ca == 1 {
    let v = (b - (a + 1)) / 2;
    let vv = c + 1 - b;
    println!("{}", 1 + v + vv);
  } else {
    println!("{}", 1 + ((c - b) / 2) + ((c - a) / 2));
  }
}
