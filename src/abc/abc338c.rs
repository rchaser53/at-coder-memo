/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::*;
use std::collections::*;

fn main() {
  input! {
    n:usize,
    q:[usize;n],
    a:[usize;n],
    b:[usize;n]
  }

  let mut result = 0;
  let limit = 10usize.pow(6);
  for i in 0..=limit {
    let mut min = 10usize.pow(6);
    let mut success = true;
    for j in 0..n {
      let mut lv = q[j];
      let av = a[j] * i;
      if lv < av {
        success = false;
        break
      }
      lv -= av;

      let bv = b[j];

      if bv != 0 {
        let num = lv / bv;
        min = min.min(num);
      }
    }

    if !success {
      break
    }
    result = result.max(i+min);
  }

  println!("{}", result);
}