/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::*;
use std::collections::*;

fn main() {
  input! {
    n:usize,
    q:usize,
    c:[Usize1;n],
    queries:[(Usize1,Usize1);q]
  }

  let mut memo = vec![HashSet::new();n];
  for i in 0..n {
    memo[i].insert(c[i]);
  }

  for (a,b) in queries {
    let mut new_set = HashSet::new();
    if memo[a].len() < memo[b].len() {
      std::mem::swap(&mut new_set, &mut memo[a]);
      for v in new_set {
        memo[b].insert(v);
      }
    } else {
      std::mem::swap(&mut new_set, &mut memo[b]);
      for v in new_set {
        memo[a].insert(v);
      }
      memo.swap(a,b);
    }
    println!("{}", memo[b].len());
  }
}