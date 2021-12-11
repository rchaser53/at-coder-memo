/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::collections::*;

fn helper(l:usize, r:usize, x:usize) -> usize {
  std::cmp::max(l.saturating_sub(x), x.saturating_sub(r))
}

fn main() {
  input! {
    n:usize,
    vals:[(usize,usize);n]
  }

  let mut a = 0;
  let mut b = usize::max_value();

  for (l,r) in vals {
    a = std::cmp::max(l,a);
    b = std::cmp::min(r,b);

    if a <= b {
      println!("0");
    } else {
      println!("{}", helper(a, b, (a+b)/2));
    }
  }
}