/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::*;
use std::collections::*;

fn main() {
  input! {
    a:isize,
    m:isize,
    l:isize,
    r:isize,
  }

  let result = if a <= l {
    let l_num = (a-l).abs();
    let mut lv = l_num / m;
    if l_num % m != 0 {
      lv += 1;
    }
    (a-r).abs() / m - lv + 1
  } else if l < a && a < r {
    (a - l).abs() / m + (a-r).abs() / m + 1
  } else {
    let r_num = (a-r).abs();
    let mut rv = r_num / m;
    if r_num % m != 0 {
      rv += 1;
    }
    (a-l).abs() / m - rv + 1
  };

  println!("{}", result);
}