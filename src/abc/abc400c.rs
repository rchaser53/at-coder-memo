
use proconio::input;
use proconio::marker::*;
use std::marker::PhantomData;
use std::cmp::*;
use std::collections::*;
use num_integer::sqrt;

fn main() {
  input! {
    n:usize,
  }

  let mut result = 0;
  let mut a_pow = 1;
  for _ in 1..=60 {
    a_pow *= 2;
    if n < a_pow {
      continue
    }
    let b = sqrt(n as u64 / a_pow as u64);
    result += (b+1)/2;
  }
  println!("{result}");
  
}