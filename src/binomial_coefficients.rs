#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::cmp::Ordering;
use std::collections::HashMap;

#[fastout]
fn main() {
  input! {
    n: usize,
    mut vals: [isize;n]
  }
  vals.sort();
  let max = vals.pop().unwrap();
  let mut half = max / 2;
  if max % 2 == 1 {
    half += 1;
  }
  let mut vals = vals
          .into_iter()
          .map(|v| {
             (
               v, 
               (half - v).abs()
             )
           })
          .collect::<Vec<(isize, isize)>>();
  vals.sort_by(|a,b| a.1.cmp(&b.1));
  println!("{} {}", max, vals.first().unwrap().0);
}