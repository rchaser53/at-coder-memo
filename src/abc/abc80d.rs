#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;
use std::collections::VecDeque;

fn main() {
  input!{
    n: usize,
    c: usize,
    mut vals: [(usize,usize,Usize1);n]
  }
  vals.sort_by(|a,b| a.0.cmp(&b.0));
  
  let mut memo = vec![0isize;210000];
  let mut channels = vec![vec![];c];
  for (from, to, i) in vals {
    if channels[i].is_empty() {
      channels[i].push((from, to));
    } else {
      let li = channels[i].len()-1;
      if channels[i][li].1 == from {
        channels[i][li].1 = to;
      } else {
        channels[i].push((from, to));
      }
    }
  }

  for i in 0..c {
    let vals = &channels[i];
    for j in 0..vals.len() {
      let (from, to) = vals[j];
      memo[2*from-1] += 1;
      memo[2*to+1] -= 1;
    }
  }

  let mut max = 0;
  let mut count = 0;
  for v in memo {
    count += v;
    max = std::cmp::max(max, count);
  }
  println!("{}", max); 
}