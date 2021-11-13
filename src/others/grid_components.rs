#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use std::cmp::Ordering;
use petgraph::unionfind::UnionFind;

#[fastout]
fn main() {
  input!{
    a: Usize1,								// 白
    b: Usize1								// 黒
  }
  
  let Limit = 100;
  let Harf = Limit / 2;
  let mut blacks = vec![vec!['#';Limit];Harf];  // 黒
  let mut whites = vec![vec!['.';Limit];Harf];	// 白

  let mut c = 0;
  let mut r = 0;
  for _ in 0..a {
    if Limit <= c {
      c = 0;
      r += 2;
    }
    blacks[r][c] = '.';
    c += 2;
  }
  
  let mut c = 0;
  let mut r = 0;
  for _ in 0..b {
    if Limit <= c {
      c = 0;
      r += 2;
    }
    whites[r][c] = '#';
    c += 2;
  }

  println!("{} {}", Limit, Limit);
  for row in blacks {
    println!("{}", row
       .into_iter()
       .map(|v| v.to_string())
       .collect::<String>()
    );
  }
  
  for i in (0..Harf).rev() {
    let row = &whites[i];
    println!("{}", row
       .into_iter()
       .map(|v| v.to_string())
       .collect::<String>()
    );
  }
}