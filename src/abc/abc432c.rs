/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::marker::PhantomData;
use std::cmp::*;
use std::collections::*;

fn main() {
  input! {
    n:usize,
    x:usize,
    y:usize,
    a:[usize;n]
  }

  let mut result = 0;
  let z = y - x;
  let min = *a.iter().min().unwrap();
  for v in a {
    if x * (v-min) % z != 0 {
      println!("-1");
      return
    }
    let swap = x * (v - min) / z;
    if swap > min {
      println!("-1");
      return
    }
    result += min - swap;
  }

  println!("{}", result);
}