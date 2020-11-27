#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use std::cmp::Ordering;
use petgraph::unionfind::UnionFind;

fn main() {
  input!{
    s: Chars,
    k: usize
  }
  
  if s.len() < k {
    println!("0");
    return
  }
  
  let mut memo = VecDeque::new();
  let mut set = HashSet::new();
  let mut count = 0;  
  for i in 0..k {
    memo.push_back(s[i]);
  }
  let v = memo.clone().into_iter().map(|v| v.to_string()).collect::<String>();
  set.insert(v);
  
  for i in k..s.len() {
    memo.pop_front();
    memo.push_back(s[i]);
    set.insert(memo.clone().into_iter().map(|v| v.to_string()).collect::<String>());
  }
  
  println!("{}", set.into_iter().collect::<Vec<String>>().len());
}