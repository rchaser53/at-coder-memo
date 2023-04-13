/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use std::cmp::Reverse;
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::collections::*;

fn main() {
  input! {
    n:usize,
    k:usize,
    mut a:[usize;n]
  }

  let a = a.into_iter().collect::<HashSet<usize>>();
  let mut a = a.into_iter().collect::<Vec<usize>>();
  a.sort();

  let mut btreeset = BTreeSet::new();
  btreeset.insert(0);
  for _ in 0..k {
    let v1 = *btreeset.iter().next().unwrap();
    btreeset.remove(&v1);
    for &v2 in &a {
      btreeset.insert(v1+v2);
    }
  }

  println!("{}", btreeset.iter().next().unwrap());
}