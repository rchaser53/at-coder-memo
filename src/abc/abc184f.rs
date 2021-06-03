/* OUTPUT FILE */
#![allow(unused_imports)]
use proconio::{input};
use proconio::marker::*;
use std::collections::*;

fn helper(
  vals: Vec<usize>,
) -> Vec<usize> {
  let n = vals.len();
  let limit = 1 << n;
  let mut set = HashSet::new();
  set.insert(0);
  for i in 0..limit {
    let mut temp = 0usize;
    for j in 0..n {
      if i >> j & 1 == 1 {
        temp += vals[j];
      }
    }
    set.insert(temp);
  }
  let mut result = set.into_iter().collect::<Vec<usize>>();
  result.sort();
  result
}

pub fn main(
) {
  input! {
    n:usize,
    t:usize,
    vals:[usize;n]
  }
  
  let hn = n / 2;
  let mut b = vec![];
  for i in hn..n {
    b.push(vals[i]);
  }
  let others = helper(b);
  let bn = others.len();

  let mut max = 0;
  let n = hn;
  let limit = 1 << n;
  for i in 0..limit {
    let mut temp = 0;
    for j in 0..n {
      if i >> j & 1 == 1 {
        temp += vals[j];
      }
    }

    if t < temp { continue }
    else if t == temp {
      println!("{}", temp);
      return
    } else {
      let left = t-temp;
      let ti = match others.binary_search(&left) {
        Ok(i) => i,
        Err(i) => i,
      };
      
      if 0 < ti && others[ti-1] <= left {
        max = std::cmp::max(max, temp+others[ti-1]);
      }

      if ti != bn && others[ti] <= left {
        max = std::cmp::max(max, temp+others[ti]);
      }

      if ti < bn-1 && others[ti+1] <= left {
        max = std::cmp::max(max, temp+others[ti+1]);
      }
    }
  }
  println!("{}", max);
}