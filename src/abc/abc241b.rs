/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::*;

fn main() {
  input! {
    n:usize,
    m:usize,
    a:[usize;n],
    b:[usize;m]
  }

  let mut map = HashMap::new();
  for v in a {
    *map.entry(v).or_insert(0) += 1;
  }

  for v in b {
    let entry = map.entry(v).or_insert(0);
    if *entry == 0 {
      println!("No");
      return
    }
    *entry -= 1;
  }

  println!("Yes");
}