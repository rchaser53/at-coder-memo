#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::{HashMap, HashSet, VecDeque};

#[fastout]
fn main() {
  input!{
    w1: usize,
    w2: usize,
    s1: usize,
    s2: usize,
    soluble: usize,
    limit: usize
  }
  
  let mut a_w = 0;
  let mut a_s = 0;
  for i in 0..=30 {
    for j in 0..=30 {
      let water = w1 * 100 * i + w2 * 100 * j;
      if limit < water { continue }
      let s_max = std::cmp::min(limit - water, soluble * water / 100);
      for x in 0..=3000 {
        let ss1 = s1 * x;
        if s_max < ss1 { break }
        let ss2 = s2 * ((s_max - ss1) / s2);
        let sugar = ss1 + ss2;
        
        if a_s == 0 {
          a_w = water;
          a_s = sugar;    
        } else {
          if sugar * (a_s + a_w) > a_s * (sugar + water) {
            a_w = water;
            a_s = sugar;
          }
        }
      }
    }
  }
  println!("{} {}", a_w + a_s, a_s);
}
