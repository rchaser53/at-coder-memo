#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::{HashMap, HashSet, VecDeque};

#[fastout]
fn main() {
  input!{
    h: usize,
    w: usize,
    n: usize,
    mut vals: [usize;n],
  }
  
  let mut vi = 0;
  let mut result: Vec<Vec<usize>> = vec![vec![0;w];h];
  for i in 0..h {
    if i % 2 == 0 {
      for ii in 0..w {
        if 0 == vals[vi] {
          vi += 1;
        }
        result[i][ii] = vi+1;
        vals[vi] -= 1;
      }
    } else {
      for ii in (0..w).rev() {
        if 0 == vals[vi] {
          vi += 1;
        }
        result[i][ii] = vi+1;
        vals[vi] -= 1;
      }
    }
  }
  for row in result {
    println!("{}", row.into_iter()
      .map(|v| v.to_string())
      .collect::<Vec<String>>()
      .join(" ")
    );
  }
}