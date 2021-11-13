#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;
use std::collections::{HashSet, HashMap, VecDeque};
 
fn main() {
  input!{
    n: usize,
    mut vals: [Chars;n]
  }
  vals.sort_by(|a, b| a.len().cmp(&b.len()));
  
  let first = vals.first().unwrap();
  let mut map: HashMap<char, usize> = HashMap::new();
  for c in first {
    let entry = map.entry(*c).or_insert(0);
    *entry += 1;
  }
  
  for i in 1..n {
    let mut temp_map: HashMap<char, usize> = HashMap::new();
    for c in vals[i].iter() {
      if let Some(v) = map.get(c) {
        let entry = temp_map.entry(*c).or_insert(0);
        *entry = std::cmp::min(*v, *entry + 1);
      }
    }
    map = temp_map;
  }
  let mut stack: Vec<String> = vec![];
  for (key, v) in map {
    for _ in 0..v {
      stack.push(key.to_string());
    }
  }
  stack.sort();
  println!("{}", stack.into_iter().collect::<String>());
}