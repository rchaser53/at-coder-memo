/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::Reverse;

fn main() {
  input! {
    mut n:usize,
  }

  let mut i = 0;
  let mut set: HashSet<usize> = HashSet::new();
  let mut tot = 0usize;
  while tot <= n {
    i += 1usize;
    tot += i;
    set.insert(i);
  }

  set.remove(&(tot-n));
  let mut result = set.into_iter().collect::<Vec<usize>>();
  result.sort();
  for v in result {
    println!("{}", v);
  }
}