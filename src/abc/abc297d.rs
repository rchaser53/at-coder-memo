/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::collections::*;

fn main() {
  input! {
    mut a:usize,
    mut b:usize,
  }
  let mut count = 0;
  loop {
    if a < b {
      let v = b / a;
      if b % a == 0 {
        println!("{}", count+v-1);
        return
      }
      count += v;
      b -= a * v;
    } else {
      let v = a / b;
      if a % b == 0 {
        println!("{}", count+v-1);
        return
      }
      count += v;
      a -= b * v;
    }
  }
}