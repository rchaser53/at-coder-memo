#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use petgraph::unionfind::UnionFind;

fn main() {
  input!{
    t: usize,
    vals: [Chars;t]
  }
  let atcoder = ['a', 't', 'c', 'o', 'd', 'e', 'r'];
  for v in vals {
    let mut all_a = true;
    for i in 0..v.len() {
      if v[i] != 'a' {
        all_a = false;
        break
      }
    }
    if all_a {
      println!("-1");
      continue
    }
    
    if 'a' < v[0] {
      println!("0");
      continue
    } else if v[1] == 'a' {
      let mut t = v.len();
      let mut a = v.len();
      for i in 2..v.len() {
        if 't' < v[i] {
          t = i-1;
          break
        }
      }
      for i in 2..v.len() {
        if 'a' < v[i] {
          a = i;
          break
        }
      }

      println!("{}", std::cmp::min(a, t));
    } else {
      let mut success = 0;
      let limit = std::cmp::min(v.len(), atcoder.len());
      for i in 0..limit {
        if atcoder[i] < v[i] {
          success = 1;
          break
        } else if v[i] < atcoder[i] {
          success = -1;
          break
        }
      }
      
      match success {
        0 => {
          if atcoder.len() < v.len() {
            println!("0");
          } else {
            println!("1");
          }
        },
        1 => { println!("0"); },
        _ => { println!("1"); }
      }
    }
  }
}