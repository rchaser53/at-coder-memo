/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::marker::PhantomData;
use std::cmp::*;
use std::collections::*;

fn main() {
  input! {
    n:usize,
    k:usize
  }

  let MOD = 1000000000usize;
  let mut v = vec![0;n+1];
  let t = if n+1 < k { n + 1 } else { k };
  for i in 0..t {
    v[i] = 1;
  }

  let mut s = 0;
  for i in 0..t {
    s = (s+v[i]) % MOD;
  }

  for i in k..=n {
    v[i] = s;
    s = (s+v[i]+MOD - v[i-k]) % MOD;
  }

  println!("{}", v[n]);
}