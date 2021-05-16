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
    m: usize,
    rows: [Chars;n]
  }
  
  let mut result = vec![vec![String::from("");m];n];
  for i in 0..n {
    for ii in 0..m {
      let mut count = 0;
      if 0 < i {
        if 0 < ii && rows[i-1][ii-1] == '#' {
          count += 1;
        }
        if rows[i-1][ii] == '#' {
          count += 1;
        }
        if ii < m-1 && rows[i-1][ii+1] == '#' {
          count += 1;
        }
      }
      
      if 0 < ii && rows[i][ii-1] == '#' {
        count += 1;
      }
      if rows[i][ii] == '#' {
        count += 1;
      }
      if ii < m-1 && rows[i][ii+1] == '#' {
        count += 1;
      }
      
      if i < n-1 {
        if 0 < ii && rows[i+1][ii-1] == '#' {
          count += 1;
        }
        if rows[i+1][ii] == '#' {
          count += 1;
        }
        if ii < m-1 && rows[i+1][ii+1] == '#' {
          count += 1;
        }
      }
      
      result[i][ii] = count.to_string();
    }
  }
  
  for v in result {
    println!("{}", v.into_iter().collect::<String>());
  }
}