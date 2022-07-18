/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::Ordering;

fn main() {
  input! {
    n:usize,
    x:usize,
    y:usize,
  }

  let mut rmemo = vec![0;n];
  let mut bmemo = vec![0;n];
  rmemo[0] = 1;

  for i in 0..n-1 {
    let v1 = rmemo[i];
    rmemo[i+1] += v1;
    bmemo[i] += v1*x;

    let v2 = bmemo[i];
    rmemo[i+1] += v2;
    bmemo[i+1] += v2 * y;
  }

  println!("{}", bmemo[n-1]);
}