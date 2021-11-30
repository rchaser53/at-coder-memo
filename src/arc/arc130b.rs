/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::Reverse;

fn main() {
  input! {
    h:usize,
    w:usize,
    c:usize,
    q:usize,
    queries:[(usize,Usize1,Usize1);q]
  }

  let mut result = vec![0;c];
  let mut hset = HashSet::new();
  let mut wset = HashSet::new();
  for (t, v, i) in queries.into_iter().rev() {
    if t == 2 {
      if !wset.contains(&v) {
        result[i] += h - hset.len();
        wset.insert(v);
      }
    }
    else {
      if !hset.contains(&v) {
        result[i] += w - wset.len();
        hset.insert(v);
      }
    }
  }
  println!("{}", result.iter().map(|v| v.to_string()).collect::<Vec<String>>().join(" "));
}