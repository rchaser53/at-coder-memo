/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::Reverse;

fn main() {
  input! {
    n:usize,
    c:f64,
    points:[(f64,f64);n]
  }

  let mut left = -100010f64;
  let mut right = 100010f64;
  for _ in 0..500 {
    // 三分探索: 極値が1つの場合使える
    // ここの2倍の場所がミスりやすい
    let p1 = (left*2f64 + right) / 3f64;
    let p2 = (left + right*2f64) / 3f64;

    let mut val1 = 0f64;
    let mut val2 = 0f64;
    for &(x, y) in &points {
      val1 += (p1-x).powi(2) + (c-y).powi(2);
      val2 += (p2-x).powi(2) + (c-y).powi(2);
    }
    if val1 < val2 {
      right = p2;
    } else {
      left = p1;
    }
  }
  let mut result = 0f64;
  for &(x, y) in &points {
    result += (left-x).powi(2) + (c-y).powi(2);
  }
  println!("{}", result);
}