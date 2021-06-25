/* OUTPUT FILE */
use proconio::input;
use proconio::marker::*;
use std::collections::*;

const MOD:usize = 1_000_000_007;

pub fn main(
) {
  input! {
    n:usize,
    k:usize,
    mut vals:[usize;n]
  }

  for x in 0..k {
    let mut memo = vec![(0,0);n];
    for i in 0..n {
      if vals[i] <= i {
        memo[i-vals[i]].0 += 1;
      } else {
        memo[0].0 += 1;
      }

      if i+vals[i] < n {
        memo[i+vals[i]].1 += 1;
      }
    }

    let mut flag = true;
    let mut temp = 0;
    for i in 0..n {
      temp += memo[i].0;
      vals[i] = temp;
      if temp != n {
        flag = false;
      }
      temp -= memo[i].1;
    }
    if flag {
      break
    }
  }

  println!("{}", vals.iter().map(|v| v.to_string()).collect::<Vec<String>>().join(" "));
}
