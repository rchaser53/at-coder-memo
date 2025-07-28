/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::*;

fn main() {
  input! {
    mut n:usize,
    m:usize,
    mut ab:[(usize, usize);m],
  }

  ab.sort_by(|a, b| {
    let v = (a.0-a.1).cmp(&(b.0-b.1));
    if v == Ordering::Equal {
      a.1.cmp(&b.1).reverse()
    } else {
      v
    }
  });
  // println!("{:?}", &ab);
  let mut count = 0;
  for (a,b) in ab {
    if a > n {
      continue;
    }
    let diff = a - b;
    let base_num = n - a;

    let x = 1 + base_num / diff;
    count += x;
    n = b;
  }
  

  println!("{}", count);
}