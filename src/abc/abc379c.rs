/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::*;

fn main() {
  input! {
    n:usize,
    m:usize,
    x:[isize;m],
    a:[isize;m]
  }

  let mut xa = vec![];
  for i in 0..m {
    xa.push((x[i],a[i]));
  }
  xa.sort();

  let mut result = 0;
  let mut now = n as isize;
  while let Some((x,a)) = xa.pop() {
    let count = now-x+1;
    if count < a {
      println!("-1");
      return
    }

    let start = now - a + 1;
    let end = now;
    let num = end - start + 1;
    let a_one = start - x;
    result += (a_one+a_one+num-1)*num/2;
    now = start-1;
  }

  if now != 0 {
    println!("-1");
  } else {
    println!("{result}");
  }
}