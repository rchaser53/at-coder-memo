#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use std::cmp::Ordering;
use petgraph::unionfind::UnionFind;

fn helper(
  memo: &mut Vec<Vec<char>>,
  temp: &mut Vec<char>,
  limit: usize,
) {
  if temp.len() == limit {
    memo.push(temp.to_vec());
    return
  }
  
  let abc = ['a', 'b', 'c'];
  for c in abc.into_iter() {
    let mut new_temp = temp.clone();
    new_temp.push(*c);
    helper(memo, &mut new_temp, limit);
  }
}

fn main() {
  input!{
    n: usize,
  }
  
  let mut result = vec![];
  helper(&mut result, &mut vec![], n);
  
  for row in result {
    let v = row
      .into_iter()
      .map(|v| v.to_string())
      .into_iter()
      .collect::<String>();
    println!("{}", v);
  }
}