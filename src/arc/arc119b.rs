/* OUTPUT FILE */
#![allow(unused_imports)]
use petgraph::algo::dijkstra;
use petgraph::graph::{DiGraph, NodeIndex, UnGraph};
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::*;

pub fn main(
) {
    input! {
      n:usize,
      mut s:Chars,
      mut t:Chars
    }

    let mut cs = vec![];
    let mut ct = vec![];
    for i in 0..n {
      if s[i] == '0' {
        cs.push(i);
      }
      if t[i] == '0' {
        ct.push(i);
      }
    }

    if cs.len() != ct.len() {
      println!("-1");
    } else {
      let mut count = 0;
      for i in 0..cs.len() {
        if cs[i] != ct[i] {
          count += 1;
        }
      }
      println!("{}", count);
    }
}
