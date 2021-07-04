/* OUTPUT FILE */
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_assignments)]

use proconio::{input};
use proconio::marker::*;
use std::collections::*;
use std::cmp::Ordering;

type Target = usize;
type UseValue = usize;

fn lower_bound(arr: &Vec<Target>, x: &UseValue) -> usize {
    let mut low = 0;
    let mut high = arr.len();
    while low != high {
        let mid = (low + high) / 2;
        // NEEDS TO EDIT
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

pub fn main(
) {
  input! {
    n: usize,
    vals: [usize;n]
  }

  let inf = n + 2;
  let mut forwards = vec![inf;n];
  let mut backs = vec![inf;n];
  let mut fmemo = vec![0;n];
  let mut bmemo = vec![0;n];

  for i in 0..n {
    let v = vals[i];
    let ti = lower_bound(&forwards, &v);
    forwards[ti] = v;
    fmemo[i] = ti + 1;
  }

  for bi in 0..n {
    let i = n - bi - 1;
    let v = vals[i];
    let ti = lower_bound(&backs, &v);
    backs[ti] = v;
    bmemo[i] = ti + 1;
  }

  let mut max = 1;
  for i in 0..n {
    max = std::cmp::max(max, fmemo[i] + bmemo[i] - 1);
  }
  println!("{}", max);
}