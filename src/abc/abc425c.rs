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
    a:[usize;n],
  }

  let mut s = 0;
  let mut memo = vec![0;2*n+10];
  for i in 0..2*n {
    memo[i+1] = a[i%n] + memo[i];
  }

  for _ in 0..q {
    input! {
      t:usize
    }

    if t == 1 {
      input! {
        c:usize
      }

      s += c;
      s %= n;

      continue
    } 

    input! {
      l:usize,
      r:usize
    }
    println!("{}", memo[s+r] - memo[s+l-1]);
  }
}