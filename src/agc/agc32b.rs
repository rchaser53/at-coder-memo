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
    n:isize,
  }

  let mut result = vec![];
  if n % 2 == 0 {
    let v = n + 1;
    for i in 1..=n {
      for j in i+1..=n {
        if i + j == v { continue }
        result.push((i, j));
      }
    }
  } else {
    let v = n;
    for i in 1..=n {
      for j in i+1..=n {
        if i + j == v { continue }
        result.push((i, j));
      }
    }
  }
  println!("{}", result.len());
  for (f, t) in result {
    println!("{} {}", f, t);
  }
}
