/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::collections::*;

fn main() {
  input! {
    n:usize,
    s:[String;n]
  }

  let mut set = HashSet::new();
  for v in s {
    let rv = v.clone();
    let mut rv = rv.chars().into_iter().collect::<Vec<char>>();
    rv.reverse();
    let rv = rv.into_iter().map(|v| v.to_string()).collect::<Vec<String>>().join("");

    if v < rv {
      set.insert(v);
    } else {
      set.insert(rv);
    }
  }
  println!("{}", set.len());
}