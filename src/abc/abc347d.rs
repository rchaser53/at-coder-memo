/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::*;

fn helper(mut a_for_c:usize, mut b_for_c:usize, mut padding: usize, c:usize) {
  let mut av = 0;
  let mut bv = 0;
  for i in 0..=60 {
    let cv = 1 << i;
    if c & cv == cv {
      if a_for_c != 0 {
        a_for_c -= 1;
        av |= cv;
      } else if b_for_c != 0 {
        b_for_c -= 1;
        bv |= cv;
      }
    } else {
      if padding != 0 {
        padding -= 1;
        av |= cv;
        bv |= cv;
      }
    }
  }

  let limit = 2usize.pow(60);
  if limit <= av || limit <= bv {
    println!("-1");
  } else {
    println!("{} {}", av, bv);
  }
}

fn main() {
  input! {
    a:usize,
    b:usize,
    c:usize
  }

  let cn = c.count_ones() as usize;
  if a + b < cn {
    println!("-1");
    return
  }

  let left = 60 - c;
  for i in 0..=a {
    let (left_a, b_for_c) = if i <= cn {
      (a - i, cn - i)
    } else {
      (a - cn, 0)
    };

    if left < left_a || b < b_for_c { continue }
    let b_for_a = b - b_for_c;
    if left_a == b_for_a {
      helper(i, b_for_c, left_a, c);
      return
    }
  }
  println!("-1");
}