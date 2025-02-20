/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use std::cmp::Reverse;
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::collections::*;

fn main() {
  input! {
    n:usize,
    m:usize,
    uv:[(Usize1,Usize1);m]
  }

  let mut result = 0;
  let mut set = HashSet::new();
  for (mut u,mut v) in uv {
    if u == v { 
      result += 1;
    } else {
      if v < u {
        std::mem::swap(&mut u, &mut v);
      }
      if set.contains(&(u, v)) {
        result += 1;
      } else {
        set.insert((u, v));
      }
    }
  }

  println!("{}", result);
}