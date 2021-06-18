/* OUTPUT FILE */
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_assignments)]

use proconio::{input};
use proconio::marker::*;
use std::collections::*;

fn culc(
  vals: &Vec<Vec<usize>>,
  nums: Vec<(usize, usize)>
) -> bool {
  for (i, j) in nums {
    if vals[i][j] != 0 {
      return false;
    }
  }
  true
}

pub fn main(
) {
  input! {
    mut vals:[[usize;3];3],
    n:usize,
    nums:[usize;n]
  }
  for v in nums {
    for i in 0..3 {
      for j in 0..3 {
        if vals[i][j] == v {
          vals[i][j] = 0;
        }
      }
    }
  }

  for i in 0..3 {
    if culc(&vals, vec![(0,i), (1, i), (2, i)]) {
      println!("Yes");
      return
    }

    if culc(&vals, vec![(i,0), (i,1), (i,2)]) {
      println!("Yes");
      return
    }
  }

  if culc(&vals, vec![(0,0), (1,1), (2,2)]) ||
    culc(&vals, vec![(0,2), (1,1), (2,0)]) {
      println!("Yes");
    } else {
      println!("No");
    }
}