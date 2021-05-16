#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use std::cmp::*;
use maplit::{btreemap, btreeset, hashmap, hashset};
use petgraph::unionfind::UnionFind;
use petgraph::algo::dijkstra;
use petgraph::graph::{NodeIndex, DiGraph, UnGraph};
use permutohedron::{Heap, heap_recursive};

const MOD:usize = 1_000_000_007;

fn main() {
  input!{
    n: usize,
    q: usize,
  }
  let mut followers = vec![vec![false;n];n];
  let mut folloyees = vec![vec![false;n];n];
  
  for _ in 0..q {
    input!{
      t: usize
    }
    
    if t == 1 {
      input!{ from: Usize1, to: Usize1 }
      followers[to][from] = true;
      folloyees[from][to] = true;
    } else if t == 2 {
      input!{ from: Usize1 }
      for ii in 0..n {
        if followers[from][ii] {
          followers[ii][from] = true;
          folloyees[from][ii] = true;
        }
      }
    } else {
      let mut temps = vec![];
      input!{ from: Usize1 }
      for x in 0..n {
        if folloyees[from][x] {
          for i in 0..n {
            if folloyees[x][i] {
              temps.push((from, i));
            }
          }
        }
      }
      
      for (from, to) in temps {
        followers[to][from] = true;
        folloyees[from][to] = true;
      }
    }
  }
  
  for i in 0..n {
    let mut result = String::from("");
    for ii in 0..n {
      if folloyees[i][ii] && i != ii {
        result = format!("{}Y", result);
      } else {
        result = format!("{}N", result);
      }
    }
    println!("{}", result);
  }
}