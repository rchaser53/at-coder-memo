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

const MOD:usize = 1_000_000_007;
const MAX: usize = 1000;

// permutation
// iterを使うと要素を並び替えてくれるやつ
// Noneになるまでnext_permutationを使えば全部の順序で試せる
// supersliceと競合するので注意
pub trait LexicalPermutation {
  fn next_permutation(&mut self) -> bool;
  fn prev_permutation(&mut self) -> bool;
}

impl<T> LexicalPermutation for [T] where T: Ord {
  fn next_permutation(&mut self) -> bool {
    if self.len() < 2 { return false; }
      let mut i = self.len() - 1;
      while i > 0 && self[i-1] >= self[i] {
        i -= 1;
      }

      if i == 0 {
        return false;
      }

      let mut j = self.len() - 1;
      while j >= i && self[j] <= self[i-1]  {
        j -= 1;
      }

      self.swap(j, i-1);

      self[i..].reverse();

      true
    }

  fn prev_permutation(&mut self) -> bool {
    if self.len() < 2 { return false; }

    let mut i = self.len() - 1;
    while i > 0 && self[i-1] <= self[i] {
      i -= 1;
    }

    if i == 0 {
      return false;
    }

    self[i..].reverse();

    let mut j = self.len() - 1;
    while j >= i && self[j-1] < self[i-1]  {
      j -= 1;
    }

    self.swap(i-1, j);

    true
  }
}

fn main() {
  input! {
    mut vals:[usize;5]
  }
  
  let mut min = 1_000_000_000;
  loop {
    let next: Vec<usize> = vals.iter().map(|v| *v).collect();
    
    let mut temp = 0;
    for i in 0..4 {
      temp += next[i];
      let a = temp % 10;
      if a != 0 {
        temp += 10 - a;
      }
    }
    temp += next[4];
    
    min = std::cmp::min(temp, min);
    if !vals.next_permutation() {
      break
    }
  } 
  println!("{}", min);
}