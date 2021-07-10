/* OUTPUT FILE */
use proconio::input;
use proconio::marker::*;
use std::collections::*;

pub fn main(
) {
  input! {
    n:usize,
  }

  for i in 1..=9 {
    if n % i == 0 && 1 <= n / i && n / i <= 9 {
      println!("Yes");
      return
    }
  }
  println!("No");
}
