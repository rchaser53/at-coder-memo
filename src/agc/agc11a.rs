/* OUTPUT FILE */
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_assignments)]
use proconio::input;
use proconio::marker::*;
use std::collections::*;

pub fn main(
) {
  input! {
    n:usize,
    c:usize,
    k:usize,
    mut vals:[usize;n]
  }
  vals.sort();

  let mut result = 0;
  let mut i = 0;
  while i < n {
    result += 1;
    let until = vals[i] + k;
    let mut temp = 0;
    loop {
      if c <= temp || n <= i || until < vals[i] {
        break
      }
      
      i += 1;
      temp += 1;
    }
  }
  println!("{}", result);
}
