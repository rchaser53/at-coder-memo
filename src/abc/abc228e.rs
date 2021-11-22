/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::Reverse;
use std::cmp::Ordering;

const MOD:usize = 998244353;

// n^pを計算するやつ
fn repeat_square(n:usize, p:usize, m:usize) -> usize {
  if p == 0 {
    1
  } else if p % 2 == 0 {
    repeat_square(n, p/2, m).pow(2) % m 
  } else {
    n * repeat_square(n, p-1, m) % m
  }
}

fn main() {
  input! {
    n:usize,
    k:usize,
    m:usize,
  }

  if m % MOD == 0 {
    println!("0");
    return
  }

  let r = repeat_square(k % (MOD-1), n, MOD-1);
  let result = repeat_square(m % MOD, r, MOD);
  println!("{}", result);

}