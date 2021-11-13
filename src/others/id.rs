#![allow(unused_imports)]
 
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::HashMap;
 
#[fastout]
fn main() {
  input! {
    n: usize,
    m: usize,
    vals: [(usize, usize);m],
  }
  let mut cities: Vec<Vec<usize>> = vec![vec![];n];
  for (i, year) in vals.iter() {
    cities[i-1].push(*year);
  }

  let mut result: HashMap<usize, usize> = HashMap::new();
  let mut index = 0;
  for i in 0..cities.len() {
    cities[i].sort();
    for (ii, v) in cities[i].iter().enumerate() {
      result.insert(*v, (i+1) * 1_000_000 + ii + 1);
    }
  }
  
  for (_, year) in vals {
    let v = result.get(&year).unwrap();
    println!("{:0>width$}", v.to_string(), width = 12); 
  }  
}