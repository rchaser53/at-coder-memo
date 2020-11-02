#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use petgraph::unionfind::UnionFind;

fn helper(
  mut v: usize,
  limit:usize
) -> Vec<Vec<usize>> {
  let mut memos:Vec<Vec<usize>> = vec![];
  while v < limit {
    let mut temp = v;
    let mut memo = vec![0;10];
    while 0 < temp {
      let i = temp % 10;
      memo[i] += 1;
      temp /= 10; 
    }

    memos.push(memo);
    v += 8;
  }
  memos
}

#[fastout]
fn main() {
  input!{
    s: Chars
  }
    
  let len = s.len();
  if len == 1 {
    if s[0] == '8' {
      println!("Yes");
    } else {
      println!("No");
    }
    return
  }
  
  let mut char_nums = vec![0;10];
  for c in s {
    let i = c.to_string().parse::<usize>().unwrap();
    char_nums[i] += 1;
  }
  
  if len == 2 {
    let twos = helper(16, 100);
    for memo in twos {
      let mut success = true;
      for i in 0..10 {
        if memo[i] > char_nums[i] {
          success = false
        }
      }
      if success {
        println!("Yes");
        return
      }
    }
  } else {
    let threes = helper(104, 1000);
    for memo in threes {
      let mut success = true;
      for i in 0..10 {
        if memo[i] > char_nums[i] {
          success = false
        }
      }
      if success {
        println!("Yes");
        return
      }
    }
  }
  
  println!("No"); 
}