/* OUTPUT FILE */
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_assignments)]

use proconio::{input};
use proconio::marker::*;
use std::collections::*;
use petgraph::unionfind::UnionFind;

pub fn main(
) {
  input! {
    n:usize
  }

  let mut map = HashMap::new();
  let mut a = n;
  let mut i = 2;
  while i * i <= n {
    if a % i == 0 {
      a /= i;
      *map.entry(i).or_insert(0) += 1;
    } else {
      i += 1;
    }
  }

  if 1 < a {
    *map.entry(a).or_insert(0) += 1;
  }

  let mut total = 0;
  for (_, num) in map {
    total += num;
  }

  let mut result = 0;
  while 2usize.pow(result) < total {
    result += 1;
  }
  println!("{}", result);
}