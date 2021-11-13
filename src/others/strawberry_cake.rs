#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use petgraph::unionfind::UnionFind;

#[fastout]
fn main() {
  input!{
    h: usize,
    w: usize,
    _k: usize,
    rows: [Chars;h]
  }
  
  let mut sss: Vec<Vec<usize>> = vec![vec![];h];
  for i in 0..h {
    let row = rows.get(i).unwrap();
    for ii in 0..w {
      if row[ii] == '#' {
        sss[i].push(ii);
      }
    }
  }
  
  let mut result:Vec<Vec<usize>> = vec![vec![0;w];h];
  let mut num = 1;
  for i in 0..h {
    let ss = sss.get(i).unwrap();
    let mut last = 0;
    for ii in 0..ss.len() {
      for iii in last..=ss[ii] {
        result[i][iii] = num;
      }
      num += 1;
      last = ss[ii] + 1;
    }
    if 0 < last && last <= w-1 {
      for ii in last-1..w {
        result[i][ii] = num-1;
      }
    }
  }
  
  for i in 0..w {
    for ii in 0..h {
      if 0 < ii && result[ii][i] == 0 {
        result[ii][i] = result[ii-1][i];
      }
    }
  }
  
  for i in 0..w {
    for ii in (0..h).rev() {
      if ii < h-1 && result[ii][i] == 0 {
        result[ii][i] = result[ii+1][i];
      }
    }
  }

  for row in result {
    println!("{}", row.into_iter()
       .map(|v| v.to_string())
       .collect::<Vec<String>>()
       .join(" "));
  }
}