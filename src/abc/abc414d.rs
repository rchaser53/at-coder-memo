/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::marker::PhantomData;
use std::cmp::*;
use std::collections::*;

fn main() {
  input! {
    n:usize,
    m:usize,
    mut x:[usize; n],
  }

  x.sort();
  let mut memo = vec![];
  for i in 0..n-1 {
    memo.push(x[i+1] - x[i]);
  }

  if m >= n-1 {
    println!("0");
    return;
  }

  memo.sort();
  let mut result = 0usize;
  for i in 0..n-m {
    result += memo[i];
  }

  println!("{}", result);
}