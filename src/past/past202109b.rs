/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::Reverse;
use std::cmp::Ordering;

fn main() {
  input!{
    n:usize,
    m:usize,
    a:[usize;n],
    b:[usize;m]
  }
  
  let mut aset = HashSet::new();
  for v in a {
    aset.insert(v);
  }
  let mut result = vec![];
  for v in b {
    if aset.contains(&v) {
      result.push(v);
    }
  }
  result.sort();

  println!("{}", result.iter().map(|v| v.to_string()).collect::<Vec<String>>().join(" "));
}