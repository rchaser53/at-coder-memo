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
    vals: [String;n]
  }
  
  let mut memo = vec![];
  for v in vals {
    let mut cv = v.clone().chars().collect::<Vec<char>>();
    cv.reverse();
    memo.push(
      (
        v,
        cv.into_iter().map(|v|v.to_string()).collect::<String>()
      )
    );
  }
    
  memo.sort_by(|a,b| a.1.cmp(&b.1));
  for (v, _) in memo {
    println!("{}", v);
  }
}