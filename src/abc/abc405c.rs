/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::marker::PhantomData;
use std::cmp::*;
use std::collections::*;
use itertools::Itertools;


fn main() {
  input! {
    n:usize,
    a:[usize;n]
  }

  let mut memo = vec![0;n+1];
  for i in (0..n).rev() {
    memo[i] = memo[i+1] + a[i];
  }

  let mut result = 0usize;
  for i in 0..n-1 {
    result += memo[i+1] * a[i];
  }

  println!("{}", result);
}