#![allow(unused_imports)]
 
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::HashMap;

fn helper(
  vals: &Vec<usize>,
  mut index: usize
) -> Vec<(usize, usize)> {
  let mut map: HashMap<usize, usize> = HashMap::new();
  while index < vals.len() {
    let entry = map.entry(vals[index]).or_insert(0);
    *entry += 1;
    index += 2;
  }
  
  let mut result: Vec<(usize, usize)> = map.into_iter().collect();
  result.sort_by(|a, b| a.1.cmp(&b.1));
  result.reverse();
  result
}

#[fastout]
fn main() {
  input! {
    n: usize,
    vals: [usize;n],
  }
  let mut result = 0;
  let mut last = vals[0];
  let mut success = true;
  for i in 1..n {
    if last != vals[i] {
      success = false;
      break
    }
  }

  if success {
    println!("{}", n / 2);
    return
  }
  
  let mut firsts = helper(&vals, 0);
  let mut seconds = helper(&vals, 1);
  if firsts.len() == 1 {
    firsts.push((10_000_000, 0));
  }
  if seconds.len() == 1 {
    seconds.push((10_000_000, 0));
  }
  
  if firsts[0].0 == seconds[0].0 {
    let result = std::cmp::min(
      n - firsts[0].1 - seconds[1].1,
      n - firsts[1].1 - seconds[0].1,
    );
    println!("{}", result);
  } else {
    println!("{}", n - firsts[0].1 - seconds[0].1);
  }
}