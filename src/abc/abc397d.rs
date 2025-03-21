/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::marker::PhantomData;
use std::cmp::*;
use std::collections::*;

fn helper(a:isize, b:isize, c:isize) -> isize {
  let mut l = 0isize;
  let mut r = 600_000_001;

  while r - l > 1 {
    let mid = (l+r)/2;
    if a * mid * mid + b * mid + c <= 0 {
      l = mid;
    } else {
      r = mid;
    }
  }

  if a * l * l + b * l + c == 0 {
    return l
  }
  -1
}

fn main() {
  input! {
    n:isize,
  }

  let mut d = 1isize;
  while d * d * d <= n {
    if n % d != 0 {
      d += 1;
      continue
    }

    let m = n / d;
    let k = helper(3, 3 * d, d*d-m);
    if k > 0 {
      println!("{} {}", k+d, k);
      return
    }

    d += 1;
  }
  println!("-1");
}