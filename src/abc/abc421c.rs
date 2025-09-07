/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::marker::PhantomData;
use std::cmp::*;
use std::collections::*;

fn main() {
  input! {
    n: usize,
    s: Chars,
  }

  let mut indices = vec![];
  for i in 0..2*n {
    if s[i] == 'A' {
      indices.push(i);
    }
  }
  let mut diff1 = 0;
  for i in 0..n {
    diff1 += (2*i).abs_diff(indices[i]);
  }
  let mut diff2 = 0;
  for i in 0..n {
    diff2 += (2*i+1).abs_diff(indices[i]);
  }

  println!("{}", min(diff1, diff2));
}