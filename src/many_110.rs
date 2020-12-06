#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use std::cmp::Ordering;
use petgraph::unionfind::UnionFind;

const MOD:usize = 1_000_000_007;

fn is_correct(
  t: &Vec<char>,
  mut i: usize
) -> bool {
  let base = vec!['1', '1', '0'];
  let mut index = 0;
  while index < t.len() {
    if base[i%3] != t[index] {
      return false
    }
    index += 1;
    i += 1;
  }
  true
}

fn main() {
  input!{
    n: usize,
    t: String
  }
  
  let tt = t.clone().chars().collect::<Vec<char>>();
  
  let mut success = (false, 0);
  for i in 0..3 {
    if is_correct(&tt, i) {
      success = (true, i);
      break
    }
  }
  
  if !success.0 {
    println!("0");
  } else if n == 1 {
    if t == String::from("1") {
      println!("{}", 10usize.pow(10) * 2);
    } else {
      println!("{}", 10usize.pow(10));
    }
  } else if n <= 2 {
    if t == String::from("01") {
      println!("{}", 10usize.pow(10) - 1);
    } else {
      println!("{}", 10usize.pow(10));
    }
  } else if t == String::from("110") {
    println!("{}", 10usize.pow(10));
  } else {
    let n = if success.1 == 0 {
      n
    } else {
      n + success.1
    };
    let mut need = n / 3;
    
    if n % 3 != 0 {
      need += 1;
    }
    println!("{}", 10usize.pow(10) - need + 1);
  }
}
