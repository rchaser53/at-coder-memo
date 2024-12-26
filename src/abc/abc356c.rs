/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::*;

fn main() {
  input! {
    n:usize,
    m:usize,
    k:usize,
  }

  let mut memo = vec![];
  for _ in 0..m {
    input! {
      c:usize,
      a:[Usize1;c],
      r:char
    }
    memo.push((a,r=='o'));
  }

  let mut result = 0;
  for i in 0..1 << n {
    let ok = memo
      .iter()
      .all(|(a,r)| (a.iter().filter(|&&v| i >> v & 1 != 0).count() >= k) == *r);

    if ok {
      result += 1;
    }
  }

  println!("{result}");
}