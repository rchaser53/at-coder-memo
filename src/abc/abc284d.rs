/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::*;

fn main() {
  input! {
    t:usize,
    ns:[usize;t]
  }
  
  for n in ns {
    let mut x = 2;
    while x * x * x <= n {
      if n % x == 0 {
        if (n / x) % x == 0 {
          let p = x;
          let q = n / p / p;
          println!("{} {}", p, q);
        } else {
          let q = x;

          let pp = n / q;
          let fpp = pp as f64;
          let p = fpp.sqrt() as usize;
          println!("{} {}", p, q);
        }
        break
      }
      x += 1;
    }
  }
}