#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use petgraph::unionfind::UnionFind;

fn main() {
  input!{
    s: Chars
  }
  let before = format!("{}{}", s[0].to_string(), s[1].to_string())
                .parse::<usize>()
                .unwrap();
  let after = format!("{}{}", s[2].to_string(), s[3].to_string())
                .parse::<usize>()
                .unwrap();
  
  // YY??
  if 0 == before || 12 < before {
    if 1 <= after && after <= 12 {
      println!("YYMM");
    } else {
      println!("NA");
    }
  }
  // MM??
  else {
    if 1 <= after && after <= 12 {
      println!("AMBIGUOUS");
    } else {
      println!("MMYY");
    }
  }
}