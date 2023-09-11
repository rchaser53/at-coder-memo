/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::*;

fn helper(l: &Vec<usize>, mid:usize) -> usize {
  let mut temp = 0;
  let mut now = 0;
  for &v in l {
    if now == 0 {
      now += v;
    } else {
      let nv = now + v + 1;
      if nv <= mid {
        now = nv;
      } else {
        temp += 1;
        now = v;
      }
    }
  }

  if 0 < now {
    temp += 1;
  }

  temp
}

fn main() {
  input! {
    n:usize,
    m:usize,
    l:[usize;n]
  }
  let base = *l.iter().max().unwrap();
  let mut min = *l.iter().max().unwrap();
  let mut max = 10usize.pow(16) + 100;

  while min + 1 < max {
    let mid = (min+max) / 2;
    let temp = helper(&l, mid);

    if temp <= m {
      max = mid;
    } else {
      min = mid;
    }
  }

  for i in min.saturating_sub(3)..=max+3 {
    if helper(&l, i) <= m {
      println!("{}", i.max(base));
      return
    }
  }
}