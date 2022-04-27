/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::*;

fn main() {
  input! {
    n:usize,
    vals:[i32;n]
  }

  let mut result = 1usize << vals[0];
  for i in 1..n {
    let lv = vals[i-1];
    let cv = vals[i];
    if lv > cv {
      result |= 1 << cv;
    } else if lv == cv {
      result += 1 << (cv+1);
    } else {
      for j in 0..cv {
        result &= !(1<<j);
      }

      if result >> cv & 1 == 1 {
        result += 1 << (cv+1);
      } else {
        result += 1 << cv;
      }
    }
  }

  println!("{}", result);
}