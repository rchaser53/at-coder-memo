/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::*;

fn main() {
  input! {
    n:usize,
    ac:[(usize,usize);n]
  }

  let mut map = HashMap::new();
  for (c,a) in ac {
    let entry = map.entry(a).or_insert(10usize.pow(10));
    if c < *entry {
      *entry = c;
    }
  }

  let mut max = 0;
  for (_, v) in map {
    max = max.max(v);
  }
  println!("{}", max);
}