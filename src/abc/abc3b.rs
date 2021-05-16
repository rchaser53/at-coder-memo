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
    s: Chars,
    t: Chars,
  }
  
  let set = hashset![
    'a', 't', 'c',
    'o', 'd', 'e', 'r'
  ];
  
  for i in 0..s.len() {
    if s[i] == t[i] { continue }
    if s[i] == '@' && set.contains(&t[i]) { continue }
    if t[i] == '@' && set.contains(&s[i]) { continue }
    println!("You will lose");
    return
  }
  println!("You can win");
}