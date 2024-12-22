/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::marker::PhantomData;
use std::cmp::*;
use std::collections::*;

fn main() {
  input! {
    n:usize,
    h:[usize;n]
  }

  let limit = 3000;
  let mut result = 1;  
  for x in 1..limit {
    let mut memo = vec![false;n];
    for i in 0..n {
      if memo[i] { continue }
      let mut temp = 1_000_000_000;
      let mut count = 0;
      for j in (i..n).step_by(x) {
        memo[j] = true;
        if h[j] != temp {
          temp = h[j];
          count = 1;
        } else {
          count += 1;
        }
        result = result.max(count);
      }
    }
  }
  println!("{}", result);
}