#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use itertools::Itertools;
use maplit::{btreemap, btreeset, hashmap, hashset};
use petgraph::unionfind::UnionFind;
use petgraph::algo::dijkstra;
use petgraph::graph::{NodeIndex, DiGraph, UnGraph};
use permutohedron::{Heap, heap_recursive};
use std::collections::*;
use std::cmp::*;

const MOD:usize = 1_000_000_007;

fn main() {
  input!{
    mut n: usize,
  }
  
  if n < 10 {
    println!("{}", n);
    return
  }
  
  let def = String::from("0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ")
    .chars().collect::<Vec<char>>();
    
  let mut result = vec!['a';3];
  result[0] = def[n / 36usize.pow(2u32)];
  n %= 36usize.pow(2u32);
  result[1] = def[n / 36usize.pow(1u32)];
  n %= 36;
  result[2] = def[n];
  let mut seen = false;
  println!("{}",
    result
      .into_iter()
      .map(|v| {
        if v == '0' && !seen {
          String::from("")
        } else {
          seen = true;
          v.to_string()
        }
      })
      .collect::<String>());
}