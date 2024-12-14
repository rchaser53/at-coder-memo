/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::marker::PhantomData;
use std::cmp::*;
use std::collections::*;

fn main() {
  input! {
    n:usize,
    a:[Usize1;n]
  }
  let mut memo = vec![vec![];n];
  for i in 0..n {
    memo[i].push(-1);
  }
  for i in 0..n {
    memo[a[i]].push(i as isize);
  }
  for i in 0..n {
    memo[i].push(n as isize);
  }

  let mut result = 0;
  for arr in &memo {
    let m = arr.len();
    if m == 2 { continue }
    result += ((1+n)*n/2) as isize;

    for i in 0..m-1 {
      let num = arr[i+1] - arr[i] - 1;
      result -= (1+num)*num/2;
    }
  }

  println!("{}", result);
}