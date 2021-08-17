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
      vals:[usize;3],
      x:usize
    }

    let mut result = 0;
    for i in 0..=vals[0] {
      for j in 0..=vals[1] {
        for k in 0..=vals[2] {
          if x == i * 500 + j * 100 + k * 50 {
            result += 1;
          }
        }
      }
    }
    println!("{}", result);
}
