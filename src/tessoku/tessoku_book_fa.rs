/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use std::collections::*;
use std::cmp::Reverse;
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;

fn main() {
  input! {
    n:usize,
    _x:isize,
    a:[isize;n-1],
    q:usize,
    queries:[(Usize1,Usize1);q]
  }

  let mut memo = vec![0;n];
  memo[0] = 0;
  for i in 0..n-1 {
    memo[i+1] = memo[i] + a[i];
  }

  for (a, b) in queries {
    let av = memo[a];
    let bv = memo[b];
    if av == bv {
      println!("Same");
    } else if av > bv {
      println!("{}", a+1);
    } else {
      println!("{}", b+1);
    }
  }
}