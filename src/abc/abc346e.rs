/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::*;

fn main() {
  input! {
    h:usize,
    w:usize,
    m:usize,
    tax:[(usize,usize,usize);m]
  }
  
  let mut map = HashMap::new();
  let mut h_set = HashSet::new();
  let mut w_set = HashSet::new();
  for (t,a,x) in tax.into_iter().rev() {
    if t == 1 {
      if !w_set.contains(&a) {
        w_set.insert(a);
        *map.entry(x).or_insert(0) += w - h_set.len();
      }
    } else {
      if !h_set.contains(&a) {
        h_set.insert(a);
        *map.entry(x).or_insert(0) += h - w_set.len();
      }
    }
  }

  let zero = (h - w_set.len()) * (w - h_set.len());
  *map.entry(0).or_insert(0) += zero;

  let mut arr = vec![];
  for (a, v) in map {
    if v > 0 {
      arr.push((a,v));
    }
  }
  arr.sort();

  println!("{}", arr.len());
  for (a, v) in arr {
    println!("{} {}", a, v);
  }
}