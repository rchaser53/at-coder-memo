/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::*;

fn main() {
  input! {
    n:usize,
    m:isize,
    mut a:[isize;n]
  }
  
  a.sort();
  let tot = a.iter().sum::<isize>();
  if tot <= m {
    println!("infinite");
    return
  }

  let mut min = 0;
  let mut max = a[n-1];
  while min+1 < max {
    let mid = (min+max) / 2;
    let mut temp = 0;
    for i in 0..n {
      temp += std::cmp::min(a[i], mid);
    }

    if m < temp {
      max = mid;
    } else {
      min = mid;
    }
  }

  println!("{}", min);
}