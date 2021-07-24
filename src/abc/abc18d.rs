/* OUTPUT FILE */

#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_assignments)]
use proconio::input;
use proconio::marker::*;
use std::collections::*;

pub fn main(
) {
  input! {
    n:usize,
    m:usize,
    p:usize,
    q:usize,
    r:usize,
    vals:[(Usize1,Usize1,usize);r]
  }

  let mut data = vec![vec![0;m];n];
  for (f, m, v) in vals {
    data[f][m] = v;
  }
  let mut max = 0;
  let limit = 1usize << n;
  let mut memo = vec![0;n];
  for i in 0..limit {
    let mut count = 0;
    for j in 0..n {
      if i >> j & 1 == 1 {
        memo[count] = j;
        count += 1;
      }
    }
    if count != p { continue }
    let mut temp = vec![0;m];
    for j in 0..p {
      let ti = memo[j];
      for k in 0..m {
        temp[k] += data[ti][k];
      }
    }
    temp.sort();
    temp.reverse();
    let mut result = 0;
    for j in 0..q {
      result += temp[j];
    }
    max = std::cmp::max(max, result);

    for i in 0..n {
      memo[i] = 0;
    }
  }
  println!("{}", max);
}
