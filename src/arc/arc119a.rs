/* OUTPUT FILE */
#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;

pub fn main(
) {
    input! {
      mut n:usize,
    }

    let mut min = std::usize::MAX;
    for b in 0..=60 {
      let v = 2usize.pow(b as u32);
      let a = n / v;
      let c = n % v;
      min = std::cmp::min(min, a+b+c);
    }

    println!("{}", min);
}
