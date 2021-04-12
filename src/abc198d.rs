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

fn culc(
  dict: &Vec<usize>,
  s: &Vec<usize>,
) -> usize {
  let mut i = 0;
  let mut result = 0;
  for ii in (0..s.len()).rev() {
    result += dict[s[ii]] * 10usize.pow(i);
    i += 1;
  }
  result
}

#[fastout]
fn main() {
  input!{
    s1:Chars,
    s2:Chars,
    s3:Chars
  }
  
  let mut set = HashSet::new();
  for i in 0..s1.len() {
    set.insert(s1[i]);
  }
  for i in 0..s2.len() {
    set.insert(s2[i]);
  }
  for i in 0..s3.len() {
    set.insert(s3[i]);
  }
  
  if 10 < set.len() {
    println!("UNSOLVABLE");
    return
  }
  
  let mut base_dict = HashMap::new();
  let mut i = 0;
  for v in set {
    base_dict.insert(v, i);
    i += 1;
  }
  let mut s1:Vec<usize> = s1.into_iter().map(|c| *base_dict.get(&c).unwrap()).collect();
  let mut s2:Vec<usize> = s2.into_iter().map(|c| *base_dict.get(&c).unwrap()).collect();
  let mut s3:Vec<usize> = s3.into_iter().map(|c| *base_dict.get(&c).unwrap()).collect();
  
  let mut random = (0..10).collect::<Vec<usize>>();
  loop {
    let temp_points: Vec<usize> = random.iter().map(|v| *v).collect();
    
    let t1 = temp_points[s1[0]];
    let t2 = temp_points[s2[0]];
    let t3 = temp_points[s3[0]];
    
    let v1 = culc(&temp_points, &s1);
    let v2 = culc(&temp_points, &s2);
    let v3 = culc(&temp_points, &s3);
    
    if t1 != 0 &&
      t2 != 0 &&
      t3 != 0 && 
      v1 + v2 == v3 {
      println!("{}", v1);
      println!("{}", v2);
      println!("{}", v3);
      return
    }
    
    if !random.next_permutation() {
      println!("UNSOLVABLE");
      return
    }
  }
}