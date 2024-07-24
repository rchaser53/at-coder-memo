/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::*;

fn main() {
  input! {
    mut n:usize,
  }

  if n == 1 {
    println!("{}", n-1);
    return
  }
  
  n -= 1;
  let mut k = 1usize;
  let d = 10usize;

  loop {
    let x = (k+1) as u32 / 2;
    let p = 9*d.pow(x-1);
    if p < n {
      n -= p;
      k += 1;
    } else {
      break
    }
  }

  let mut result = vec![0;k];
  let mut i = 0;
  while 0 < k {
    let j = result.len()  - 1 - i;
    let x = (k+1) as u32 / 2;
    let t = d.pow(x-1);
    if i == 0 {
      result[i] = (n-1)/t+1;
      result[j] = (n-1)/t+1;
    } else if n == 0 {
      result[i] = 9;
      result[j] = 9;
    } else {
      result[i] = (n-1)/t;
      result[j] = (n-1)/t;
    }

    n %= t;
    if k <= 2 {
      break
    } else {
      k -= 2;
      i += 1;
    }
  }

  for a in result {
    print!("{}", a);
  }
  

  // unreachable!{}
}