#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::{HashMap, HashSet, VecDeque};

struct Helper {
  dp: Vec<Vec<usize>>,
  set: HashSet<String>
}

impl Helper {
  fn culc(
    &mut self,
    val: String,
    index: usize
  ) {
    if val.len() == 3 {
      self.set.insert(val);
      return
    }
  
    for i in 0..10 {
      for ii in 0..self.dp[i].len() {
        let new_index = self.dp[i][ii];
        if index <= new_index {
          continue
        } else {
          self.culc(format!("{}{}", val, i.to_string()), new_index);
          break
        }
      }
    }
  }
}

#[fastout]
fn main() {
  input!{
    n: usize,
    s: Chars,
  }
  
  let mut dp = vec![vec![];10];
  for i in 0..n {
    let c = s[i];
    let ii = c.to_string().parse::<usize>().unwrap();
    dp[ii].push(i);
  }
  for i in 0..10 {
    dp[i].reverse();
  }
  
  let mut set:HashSet<String> = HashSet::new();
  let mut helper = Helper { dp, set };
  
  for i in 0..10 {
    let index = if let Some(index) = helper.dp[i].first() {
      *index
    } else {
      continue;
    };
    helper.culc(i.to_string(), index);
  }
  println!("{}", helper.set.len());
}