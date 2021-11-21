/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::Reverse;
use std::cmp::Ordering;

fn main() {
  input! {
    q:usize,
    queries:[(usize,isize);q]
  }

  let n = 2usize.pow(20);
  let iin = n as isize;
  let mut memo = vec![-1isize;n];
  let mut map = HashMap::new();

  for (t, v) in queries {
    if t == 2 {
      let i = (v % iin) as usize;
      println!("{}", memo[i]);
      continue
    }

    let mut h = (v % iin) as usize;
    let mut stack = vec![];
    while memo[h] != -1 {
      stack.push(h);
      if let Some(goal) = map.get(&h) {
        h = *goal + 1;
        h %= n;
      } else {
        h += 1;
        h %= n;
      }
    }

    memo[h] = v;
    for v in stack {
      map.insert(v, h);
    }
  }
}