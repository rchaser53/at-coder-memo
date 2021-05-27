/* OUTPUT FILE */
#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::Reverse;

type Target = usize;
type UseValue = usize;
fn upper_bound(arr: &Vec<Target>, x: &UseValue) -> usize {
    let mut low = 0;
    let mut high = arr.len();
    while low != high {
        let mid = (low + high) / 2;
        // NEEDS TO EDIT
        match arr[mid].cmp(x) {
            std::cmp::Ordering::Less | std::cmp::Ordering::Equal => {
                low = mid + 1;
            }
            std::cmp::Ordering::Greater => {
                high = mid;
            }
        }
    }
    low
}

pub fn main(
) {
  input! {
    n:usize,
    k:usize,
    p:usize,
    vals:[usize;n]
  }

  let ln = n / 2;
  let mut rn = n / 2;
  if n % 2 == 1 {
    rn += 1;
  }
  
  let mut ls = vec![vec![];ln+1];
  let mut rs = vec![vec![];rn+1];

  let limit = 1 << ln;
  for i in 0..limit {
    let mut count = 0usize;
    let mut temp = 0usize;
    for j in 0..ln {
      if i >> j & 1 == 1 {
        temp += vals[j];
        count += 1;
      }
    }
    ls[count].push(temp);
  }

  let limit = 1 << rn;
  for i in 0..limit {
    let mut count = 0usize;
    let mut temp = 0usize;
    for j in 0..rn {
      if i >> j & 1 == 1 {
        temp += vals[ln+j];
        count += 1;
      }
    }
    rs[count].push(temp);
  }

  for i in 0..rs.len() {
    rs[i].sort();
  }

  let mut result = 0usize;
  for i in 0..ls.len() {
    if k < i { break }
    let rc = k - i;
    if rs.len() <= rc { continue }
    for &v in &ls[i] {
      if p < v { continue }
      let rv = p - v;
      let a = upper_bound(&rs[rc], &rv);
      result += a;
    }
  }
  println!("{}", result);
}
