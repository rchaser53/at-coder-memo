/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use std::cmp::Reverse;
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::collections::*;


fn main() {
  input! {
    n:usize,
    mut k:usize,
    mut a:[usize;n]
  }

  let mut l = 0;
  let mut r = 10usize.pow(12) + 10;
  while l + 1 < r {
    let mid = (l+r) / 2;
    let mut count = 0;
    let mut ca = a.clone();
    for i in 0..n {
      count += std::cmp::min(mid, ca[i]);
      ca[i] = ca[i].saturating_sub(mid);
    }

    if count == k {
      println!("{}", ca.into_iter().map(|v| v.to_string()).collect::<Vec<String>>().join(" "));
      return
    }

    for i in 0..n {
      if 0 < ca[i] {
        ca[i] -= 1;
        count += 1;
      }

      if count == k {
        println!("{}", ca.into_iter().map(|v| v.to_string()).collect::<Vec<String>>().join(" "));
        return
      }
    }

    if k < count {
      r = mid;
    } else {
      l = mid;
    }
  }

  for i in 0..n {
    k -= std::cmp::min(l, a[i]);
    a[i] = a[i].saturating_sub(l);
  }

  let mut i = 0;
  while i < n && 0 < k {
    if 0 < a[i] {
      a[i] -= 1;
      k -= 1;
    }

    i += 1;
  }
  println!("{}", a.into_iter().map(|v| v.to_string()).collect::<Vec<String>>().join(" "));
}