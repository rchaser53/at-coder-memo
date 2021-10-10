use proconio::input;
use proconio::marker::*;
use std::collections::*;

fn main() {
  input! {
    n:usize,
    vals:[usize;n]
  }

  let mut odd = 1usize;
  let total = 3usize.pow(n as u32);
  for i in 0..n {
    let v = vals[i];
    if v % 2 == 0 {
      odd *= 2;
    }
  }
  println!("{}", total - odd);
}