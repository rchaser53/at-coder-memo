/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use std::cmp::Reverse;
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::collections::*;

fn helper(a: &Vec<usize>, fv:usize) -> Option<i32> {
  let n = a.len();
  let mut count = 0;
  for j in 1..n {
    let mut v = a[j];

    if v % fv != 0 {
      return None
    }

    v /= fv;
    while v % 2 == 0 {
      v /= 2;
      count += 1;
    }

    while v % 3 == 0 {
      v /= 3;
      count += 1;
    }

    if v != 1 {
      return None
    }
  }
  Some(count)
}

fn main() {
  input! {
    n:usize,
    mut vals:[usize;n]
  }

  vals.sort();
  let mut result = i32::max_value();
  let fv = vals[0];
  let mut pattern = vec![];
  let limit = 10usize.pow(9);
  for i in 0..30 {
    let v1 = 2usize.pow(i);
    for j in 0..30 {
      let v2 = 3usize.pow(j);
      if limit < v1 && limit < v2 { continue }
      pattern.push(((i+j) as i32, v1*v2));
    }
  }

  for (num, i) in pattern {
    if i <= fv && fv % i == 0 {
      if let Some(v) = helper(&vals, i) {
        result = result.min(v+num);
      }
      if let Some(v) = helper(&vals, fv/i) {
        result = result.min(v+num);
      }
    }
  }

  if result == i32::max_value() {
    println!("-1");
  } else {
    println!("{}", result);
  }
}