#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;

const MOD:usize = 1_000_000_007;
#[fastout]
fn main() {
  input!{
    n: usize,
    vals: [isize;n],
    m: usize,
    drinks: [(Usize1, isize);m]
  }
  
  let base = vals.iter().sum::<isize>();
  for (ti, v) in drinks {
    println!("{}", base - vals[ti] + v);
  }
}