/* OUTPUT FILE */
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_assignments)]

use proconio::{input};
use proconio::marker::*;
use std::collections::*;

pub fn main(
) {
  input! {
    r:usize,
    c:usize,
    k:usize,
    n:usize,
    vals:[(Usize1,Usize1);n]
  }
  
  let mut row_sum = vec![0;r];
  let mut col_sum = vec![0;c];
  for &(r, c) in &vals {
    row_sum[r] += 1;
    col_sum[c] += 1;
  }

  let mut row_sum_count = vec![0;k+1];
  let mut col_sum_count = vec![0;k+1];
  for &cnt in &row_sum {
    if cnt <= k {
      row_sum_count[cnt] += 1;
    }
  }
  for &cnt in &col_sum {
    if cnt <= k {
      col_sum_count[cnt] += 1;
    }
  }

  let mut result = 0usize;
  for i in 0..k+1 {
    result += row_sum_count[i] * col_sum_count[k-i];
  }

  for (r, c) in vals {
    let temp = row_sum[r] + col_sum[c];
    if temp == k + 1 {
      result += 1;
    } else if temp == k {
      result -= 1;
    }
  }

  println!("{}", result);
}