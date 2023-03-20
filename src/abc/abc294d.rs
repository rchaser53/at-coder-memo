/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use std::cmp::Reverse;
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::collections::*;

fn main() {
  input! {
    n:usize,
    q:usize
  }

  let mut btreemap = BTreeMap::new();
  let mut i = 1;
  let mut result = vec![];
  for _ in 0..q {
    input! {
      t:usize
    }

    if t == 1 {
      btreemap.insert(i,false);
      i += 1;
    } else if t == 2 {
      input! {
        j: usize
      }
      btreemap.remove(&j).unwrap();
    } else {
      // *btreemap.iter_mut().next().unwrap().1 = true;
      if let Some(j) = btreemap.iter().next() {
        result.push(*j.0);
      }
    }
  }

  for v in result {
    println!("{}", v);
  }
}
