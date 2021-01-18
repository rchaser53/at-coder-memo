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

#[fastout]
fn main() {
  let mut s = String::new();
  std::io::stdin().read_line(&mut s).unwrap();
  s = s.trim().to_string();
  let s = s.chars().collect::<Vec<char>>();
  
  input!{
    n: usize,
    ngs: [Chars;n]
  }
  
  let mut words = vec![vec![]];
  for c in s {
    if c == ' ' {
      words.push(vec![]); 
    } else {
      let li = words.len()-1;
      words[li].push(c);
    }
  }
  
  let mut map = HashMap::new();
  for ng in ngs {
    let li = ng.len();
    let mut entry = map.entry(li).or_insert(vec![]);
    entry.push(ng);
  }
  
  let mut result = vec![];
  for word in words {
    let wl = word.len();
    if let Some(dict) = map.get(&wl) {
      let mut success = false;
      for ngw in dict.iter() {
        success = true;
        for i in 0..wl {
          if ngw[i] != '*' && ngw[i] != word[i] {
            success = false;
            break
          }
        }
        if success { break }
      }
      
      if success {
        result.push(
          vec![String::from("*");wl].into_iter().collect::<String>()
        )
      } else {
        result.push(
          word.into_iter().map(|v| v.to_string()).collect::<String>()
        );
      }
    } else {
      result.push(
        word.into_iter().map(|v| v.to_string()).collect::<String>()
      );
    }
  }
  
  println!("{}", result.join(" "));
}