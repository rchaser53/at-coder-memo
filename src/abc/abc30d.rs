#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use std::cmp::Ordering;
use petgraph::unionfind::UnionFind;

const MOD:usize = 1_000_000_007;

fn main() {
  input!{
    n: usize,
    a: Usize1,
    k: Chars,
    b: [Usize1; n]
  }
  
  let mut memo = vec![None;n];
  let mut words = vec![];
  let mut pos = a;
  let mut count = 0usize;
  let (base, m) = loop {
    if let Some(id) = memo[pos] {
      break (id, count - id);
    } else {
      memo[pos] = Some(count);
      words.push(pos);
      pos = b[pos];
      count += 1;
    }
  };
  
  let mut d = 0;
  let mut over = false;
  for c in k {
    let c = c.to_digit(10).unwrap() as usize;
    d = 10 * d + c;
    if !over && d >= words.len() {
      over = true;
    }
    if over {
      d %= m;
    }
  }
  
  if !over {
    println!("{}", words[d] + 1);
    return
  }
  
  d = (d + m*base - base) % m;
  println!("{}", words[d+ base] + 1);
}
