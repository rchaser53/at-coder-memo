#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use std::cmp::Ordering;
use maplit::{btreemap, btreeset, hashmap, hashset};
use petgraph::unionfind::UnionFind;
use petgraph::algo::dijkstra;
use petgraph::graph::{NodeIndex, UnGraph};

const MOD:usize = 1_000_000_007;

fn main() {
  input!{
    n: usize,
    mut s: Chars
  }
  
  let mut stack = vec![];
  for i in 0..n {
    if s[i] == '#' {
      stack.push(i);
    }
  }
  
  if stack.is_empty() {
    println!("0");
    return
  }
  
  let mut left = *stack.first().unwrap();
  let mut right = n - *stack.last().unwrap() - 1;
  
  for i in 0..stack.len() {
    let v = stack[i];
    for ii in 1..=left {
      let iii = v.saturating_sub(ii);
      s[iii] = '#';
    }
    
    let mut ii = v;
    while ii < n && ii <= v+right {
      s[ii] = '#';
      ii += 1;
    }
  }
  
  let mut max = 0;
  let mut temp = 0;
  for i in 0..n {
    if s[i] == '#' {
      max = std::cmp::max(temp, max);
      temp = 0;
    } else {
      temp += 1;
    }
  }
  max = std::cmp::max(temp, max);
  
  println!("{} {}", left + max, right);
}