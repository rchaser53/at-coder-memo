use proconio::input;
use proconio::marker::*;
use std::collections::*;

fn main() {
  input!{
    n:usize,
    vals:[usize;n]
  }

  let mut map = HashMap::new();
  for v in vals {
    *map.entry(v).or_insert(0) += 1;
  }

  if map.keys().len() == 1 {
    if let Some(_) = map.get(&0) {
      println!("Yes");
    } else {
      println!("No");
    }
    return
  }
  
  if map.keys().len() == 2usize {
    if let Some(v) = map.get(&0) {
      if v * 2usize == n || v * 3usize == n{
        println!("Yes");
        return
      }
    }
    println!("No");
    return
  }

  if map.keys().len() != 3 {
    println!("No");
    return
  }

  let mut memo = vec![];
  for (key, val) in map {
    memo.push((key, val));
  }

  let a = memo[0].1 == memo[1].1 
    && memo[1].1 == memo[2].1
    && memo[2].1 == memo[0].1;

  if a
    && memo[0].0 ^ memo[2].0 == memo[1].0 
    && memo[1].0 ^ memo[0].0 == memo[2].0 
    && memo[1].0 ^ memo[2].0 == memo[0].0 {
      println!("Yes");
    } else {
      println!("No");
    }
}