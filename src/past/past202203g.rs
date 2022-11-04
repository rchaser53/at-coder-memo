/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use std::cmp::Reverse;
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::collections::*;
  
fn main() {
  input! {
    a:f64,
    b:f64,
    c:f64
  }

  let border = 0.000_000_000_001;
  let mut l = 1.000_000_000_001f64;
  let mut r = 1.999_999_999_999f64;

  let c = c.abs();
  for _ in 0..1000 {
    let mid = (l+r) / 2.0;
    let v = a * mid.powi(5) + b * mid;
    if (c-v).abs() < border {
      println!("{:.10}", mid);
      return
    } else if v < c {
      l = mid;
    } else {
      r = mid;
    }
  }
  
  println!("{:.10}", l);
}