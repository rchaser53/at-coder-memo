use proconio::input;
use proconio::marker::*;
use std::collections::*;

fn main() {
  input!{
    vals1:[usize;2],
    vals2:[usize;2],
  }

  if vals1[0] == vals2[0]
    || vals1[0] == vals2[1]
    || vals1[1] == vals2[0]
    || vals1[1] == vals2[1] {
      println!("YES");
    } else {
      println!("NO");
    }
}