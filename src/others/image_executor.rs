#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use std::cmp::Ordering;
use petgraph::unionfind::UnionFind;

fn main() {
  input!{
    h: usize,
    w: usize,
    rows: [Chars;h]
  }
  
  let mut memo = vec![vec![false;w];h];
  let mut result = vec![vec![String::from(".");w];h];
  for i in 0..h {
    for ii in 0..w {
      if rows[i][ii] == '.' { continue }
      if 0 < i {
        if rows[i-1][ii] == '.' { continue }
        if 0 < ii && rows[i-1][ii-1] == '.' { continue }
        if ii < w-1 && rows[i-1][ii+1] == '.' { continue }
      }
      
      if 0 < ii && rows[i][ii-1] == '.' { continue }
      if ii < w-1 && rows[i][ii+1] == '.' { continue }
      
      if i < h-1 {
        if rows[i+1][ii] == '.' { continue }
        if 0 < ii && rows[i+1][ii-1] == '.' { continue }
        if ii < w-1 && rows[i+1][ii+1] == '.' { continue }
      }
      memo[i][ii] = true;
    }
  }
  
  for i in 0..h {
    for ii in 0..w {
      if !memo[i][ii] { continue }
      if 0 < i {
        result[i-1][ii] = String::from("#");
        if 0 < ii { result[i-1][ii-1] = String::from("#"); }
        if ii < w-1 { result[i-1][ii+1] = String::from("#"); }
      }
      
      result[i][ii] = String::from("#");
      if 0 < ii { result[i][ii-1] = String::from("#"); }
      if ii < w-1 { result[i][ii+1] = String::from("#"); }
      
      if i < h-1 {
        result[i+1][ii] = String::from("#");
        if 0 < ii { result[i+1][ii-1] = String::from("#"); }
        if ii < w-1 { result[i+1][ii+1] = String::from("#"); }
      }
    }
  }

  for i in 0..h {
    let v = result[i]
      .iter()
      .map(|v| v.to_string())
      .collect::<String>();
    let vv = rows[i]
      .iter()
      .map(|v| v.to_string())
      .collect::<String>();
    if v != vv {
      println!("impossible");
      return
    }
  }
  
  println!("possible");
  for i in 0..h {
    let row = memo[i]
      .iter()
      .map(|v| if *v {
        String::from("#")
      } else {
        String::from(".")
      })
      .collect::<String>();
    println!("{}", row);
  }
}