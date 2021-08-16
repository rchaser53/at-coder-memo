/* OUTPUT FILE */
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_assignments)]

use proconio::input;
use proconio::marker::*;
use std::collections::*;

pub fn main(
) {
    input! {
      n:usize,
      mut vals:[isize;n],
    }

    vals.sort();
    vals.reverse();
    let mut a = 0isize;
    let mut b = 0isize;
    for i in 0..n {
      let v = vals[i];
      if i % 2 == 0 {
        a += v;
      } else {
        b += v;
      }
    }
    
    println!("{}", a - b);
}
