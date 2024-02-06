/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::*;
use std::collections::*;

fn main() {
  input! {
    n:usize,
    m:usize,
    x:[Usize1;m]
  }

  let mut memo = vec![0;n+1];
  let mut memo2 = vec![(0,0);n+1];
  let mut now = 0;

  for i in 0..m-1 {
    let mut a = x[i];
    let mut b = x[i+1];
    if a > b {
      std::mem::swap(&mut a, &mut b);
    }
    let ab = b - a;
    let ba = n - b + a;

    memo2[0].0 += ab;
    memo2[a].1 += ab;
    memo2[b+1].0 += ab;

    memo2[a+1].0 += ba;
    memo2[b].1 += ba;
  }

  for i in 0..=n {
    now += memo2[i].0;
    memo[i] = now;
    now -= memo2[i].1;
  }

  println!("{}", memo.into_iter().min().unwrap());

}