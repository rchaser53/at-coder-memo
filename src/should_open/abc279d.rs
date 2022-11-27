/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use std::cmp::Reverse;
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::collections::*;

// https://atcoder.jp/contests/abc279/tasks/abc279_d
fn main() {
  input! {
    a:usize,
    b:usize
  }

  let fa = a as f64;
  let fb = b as f64;
  let mut left = 0;
  let mut right = a+10;
  while left + 2 < right {
    // 三分探索: 極値が1つの場合使える
    // ここの2倍の場所がミスりやすい
    let time1 = (left * 2 + right) / 3;
    let time2 = (left + right * 2) / 3;

    let g1 = (time1 as f64 + 1f64).sqrt();
    let g2 = (time2 as f64 + 1f64).sqrt();

    let val1 = fa / g1 + (time1 as f64 - 1f64) * fb;
    let val2 = fa / g2 + (time2 as f64 - 1f64) * fb;
    
    // 極値が下に凸の場合、大きい方を減らす
    if val1 >= val2 {
      left = time1;
    } else {
      right = time2;
    }
  }

  let mut min = fa + 10f64;
  for i in left..=right {
    let v = fa / (i as f64 + 1f64).sqrt() + i as f64 * fb;
    if v < min {
      min = v;
    }
  }
  println!("{:.7}", min);
}