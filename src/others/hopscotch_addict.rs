#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
 
#[fastout]
fn main() {
  input!{
    n: usize,
    m: usize,
    vals: [(Usize1, Usize1);m],
    s: Usize1,
    t: Usize1
  }
  
  let mut memo: Vec<Vec<usize>> = vec![vec![];n];
  for (from, to) in vals {
    memo[from].push(to);
  }
  let mut seen:HashSet<(usize,usize)> = HashSet::new();
  let mut deque:VecDeque<(usize, usize)> = VecDeque::new();
  deque.push_back((s, 0));  

  let mut ans = usize::max_value();
  while !deque.is_empty() {
    let (index, count) = deque.pop_front().unwrap();
    let c = count % 3;
    if index == t && c == 0 {
      ans = std::cmp::min(ans, count / 3);
      continue
    }
    
    if seen.contains(&(index, c)) {
      continue
    }
    seen.insert((index, c));
    
    for i in 0..memo[index].len() {
      let to = memo[index][i];
      deque.push_back((to, count+1));
    }
  }

  if ans == usize::max_value() {
    println!("-1");
  } else {
    println!("{}", ans);
  }
}