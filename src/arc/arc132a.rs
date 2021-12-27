/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::collections::*;

fn main() {
  input! {
    n:usize,
    rs:[usize;n],
    cs:[usize;n],
    q:usize,
    queries:[(Usize1,Usize1);q]
  }

  for (r, c) in queries {
    if n < rs[r] + cs[c] {
      print!("#")
    } else {
      print!(".")
    }
  }
  print!("\n")
}