/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::*;

type Target = usize;
type UseValue = usize;
fn lower_bound(arr: &Vec<Target>, x: &UseValue) -> usize {
  let mut low = 0;
  let mut high = arr.len();
  while low != high {
    let mid = (low + high) / 2;
    match arr[mid].cmp(x) {
      std::cmp::Ordering::Less => {
        low = mid + 1;
      }
      std::cmp::Ordering::Equal | std::cmp::Ordering::Greater => {
        high = mid;
      }
    }
  }
  low
}

fn main() {
  input! {
    n:usize,
    q:usize,
    mut r:[usize;n],
    queries:[usize;q]
  }

  r.sort();
  let mut memo = vec![0;n+1];
  for i in 0..n {
    memo[i+1] = memo[i] + r[i];
  }

  for v in queries {
    let i = lower_bound(&memo, &v);
    if i > n {
      println!("{}", n);
    } else if memo[i] == v {
      println!("{}", i);
    } else if i == 0 {
      println!("{}", 0);
    } else {
      println!("{}", i-1);
    }
  }
}