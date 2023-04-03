/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::*;

fn main() {
  input!{
    n:usize,
    x:isize,
    a:[isize;n]
  }

  let mut set = HashSet::new();
  for i in 0..n {
    set.insert(a[i]);
  }

  for v in a {
    if set.contains(&(v-x)) {
      println!("Yes");
      return
    }
  }
  println!("No");
}