/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::marker::PhantomData;
use std::cmp::*;
use std::collections::*;

fn main() {
  input! {
    n:usize,
    q:usize,
    mut a:[isize;n],
    mut b:[isize;n],
    cxv:[(char,Usize1,isize);q]
  }

  let mut sum = 0isize;
  for i in 0..n {
    sum += std::cmp::min(a[i], b[i]);
  }

  for (c,x,v) in cxv {
    if a[x] < b[x] {
      sum -= a[x];
    } else {
      sum -= b[x];
    }

    if c == 'A' {
      a[x] = v;
    } else {
      b[x] = v;
    }
    sum += std::cmp::min(a[x], b[x]);

    println!("{}", sum);
  }
}