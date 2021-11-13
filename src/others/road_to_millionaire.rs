#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;

#[fastout]
fn main() {
  input!{
    n: usize,
    vals: [usize;n]
  }
  
  let mut hold = (0, 1000);
  let mut buy = (0, 1000);
  
  for v in vals {
    let new_buy = (hold.1 / v, hold.1 % v);
    let sell_score = buy.0 * v + buy.1;
    if hold.1 < sell_score {
      hold = (0, sell_score);
    }
    
    if buy.0 < new_buy.0 {
      buy = new_buy;
    } else if buy.0 == new_buy.0 && buy.1 < new_buy.1 {
      buy = new_buy;   
    }
  }
  println!("{}", hold.1);
}