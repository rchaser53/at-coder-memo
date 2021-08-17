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
      vals:[usize;n]
    }

    if vals[0] != 0 {
      println!("-1");
      return
    }

    let mut max = 0;
    let mut i = n - 1;
    let mut result = 0;
    while 0 < i {
      let v = vals[i];
      if v < max {
        println!("-1");
        return
      } else if v == max {
        i -= 1;
        if 0 < max {
          max -= 1;
        }
        continue
      }
      max = v;

      if i < v {
        println!("-1");
        return
      }
      result += v;
      i -= 1;
      if 0 < max {
        max -= 1;
      }
    }
    println!("{}", result);
}
