/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::*;


fn main() {
  input! {
    n:usize,
    rows:[Chars;n]
  }

  let mut cs = vec![0;n];
  let mut rs = vec![0;n];
  for i in 0..n {
    for j in 0..n {
      if rows[i][j] == 'o' {
        rs[i] += 1;
        cs[j] += 1;
      }
    }
  }

  let mut result = 0usize;
  for i in 0..n {
    for j in 0..n {
      if rows[i][j] == 'o' {
        result += (rs[i]-1) * (cs[j] - 1);
      }
    }
  }
  println!("{}", result);

}