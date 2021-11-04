/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::Reverse;

fn main() {
  input! {
    n:usize,
    k:usize,
    colors:[usize;n],
    vals:[usize;n]
  }
  let inf = 1_000_000_000_000;
  let mut map = HashMap::new();
  for i in 0..n {
    *map.entry(colors[i]).or_insert(inf) = std::cmp::min(
      *map.entry(colors[i]).or_insert(inf),
      vals[i]
    );
  }

  if map.keys().len() < k {
    println!("-1");
    return
  }

  let mut memo = vec![];
  for (_, v) in map {
    memo.push(v);
  }
  memo.sort();
  let mut result = 0usize;
  for i in 0..k {
    result += memo[i];
  }
  println!("{}", result);
}