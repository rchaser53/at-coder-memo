/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::*;

fn main() {
  input! {
    q:usize
  }

  let mut map = HashMap::new();
  for _ in 0..q {
    input! {
      t:usize
    }

    if t == 1 {
      input! {
        v:usize
      }
      *map.entry(v).or_insert(0) += 1;
    } else if t == 2{
      input! {
        v:usize
      }
      let entry = map.entry(v).or_insert(0);
      *entry -=1;
      if *entry == 0 {
        map.remove(&v);
      }

    } else {
      println!("{}", map.keys().len());
    }
  }
}