#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use std::cmp::Ordering;
use maplit::{btreemap, btreeset, hashmap, hashset};
use petgraph::unionfind::UnionFind;
use petgraph::algo::dijkstra;
use petgraph::graph::{NodeIndex, UnGraph};
use num::Num;

const MOD:usize = 1_000_000_007;

fn main() {
  let mut s = String::new();
  std::io::stdin().read_line(&mut s).unwrap();
  s = s.trim().to_string();
  let s = s.split(" ")
   .into_iter()
   .map(|v| v.trim())
   .collect::<Vec<&str>>();
  let mut result = vec![];
  for v in s {
    match v {
      "Left" => result.push(String::from("<")),
      "Right" => result.push(String::from(">")),
      _ => result.push(String::from("A")),
    }
  }
  println!("{}", result.join(" "));
}
