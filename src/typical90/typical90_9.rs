/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::*;
use std::f64::consts::PI;

fn main() {
  input! {
    n:usize,
    xy:[(f64,f64);n]
  }

  let mut result = 0.0;
  for i in 0..n {
    let (p,q) = xy[i];
    let mut arr = xy.clone();
    arr.swap_remove(i);
    let mut arr = arr.into_iter()
      .map(|(x,y)| (y-q).atan2(x-p))
      .collect::<Vec<f64>>();
    arr.sort_by(|&a,&b| {
      let a = if a < 0.0 { a+PI } else { a };
      let b = if b < 0.0 { b+PI } else { b };
      a.partial_cmp(&b).unwrap()
    });
    for j in 0..arr.len() {
      let d = (arr[j]-arr[(j+1)%arr.len()]).abs();
      result = (if d > PI {
        PI*2.0 - d 
      } else {
        d
      }).max(result);
    }
  }

  println!("{}", result*180.0/PI);
}