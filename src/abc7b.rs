#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use std::cmp::Ordering;
use petgraph::unionfind::UnionFind;
use petgraph::algo::dijkstra;
use petgraph::graph::{NodeIndex, UnGraph};
use std::ops::Bound::Included;

fn main() {
  input!{
    mut s:Chars
  }
  
  if s[0] == 'a' {
    if s.len() == 1 {
      println!("-1");
    } else {
      let mut result = String::from("");
      for i in 0..s.len()-1 {
        result = format!("{}{}", result, s[i]);
      }
      println!("{}", result);
    }
  } else {
    s[0] = 'a';
    println!("{}", s
      .into_iter()
      .map(|v| v.to_string())
      .collect::<String>()
    );
  }
  
}