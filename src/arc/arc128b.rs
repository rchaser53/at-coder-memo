/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::Reverse;

fn main() {
  input! {
    t:usize,
    vals:[[usize;3];t]
  }

  for mut v in vals {
    v.sort();
    if (v[1] - v[0]) % 3 == 0 {
      println!("{}", v[1]);
    } else if (v[2]-v[1]) % 3 == 0 || (v[2]-v[0]) % 3 == 0 {
      println!("{}", v[2]);
    } else {
      println!("-1");
    }
  }
}