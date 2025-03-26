/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::marker::PhantomData;
use std::cmp::*;
use std::collections::*;

fn main() {
  input! {
    n:usize,
    a:[isize;n]
  }

  let mut map = HashMap::new();
  for i in 0..n {
    *map.entry(a[i]).or_insert(0) += 1;
  }

  let mut max = -1;
  for (k, v) in map {
    if v == 1 {
      max = max.max(k);
    }
  }

  for i in 0..n {
    if a[i] == max {
      println!("{}", i + 1);
      return
    }
  }
  println!("{max}");
}