#![allow(unused_imports)]
 
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::HashMap;

fn gcv(a: usize, b:usize) -> usize {
  if b == 0 {
    a
  } else {
    gcv(b, a % b)
  }
}

#[fastout]
fn main() {
  input! {
    n: usize,
    x: usize,
    mut arr: [usize;n],
  }
  
  arr.push(x);
  arr.sort();
  
  let mut vals: Vec<usize> = vec![0;n];
  for i in 1..n+1 {
    vals[i-1] = arr[i] - arr[i-1];
  }
  
  let mut v = vals[0];
  for i in 0..vals.len() {
    v = gcv(v, vals[i]);
  }
  
  println!("{}", v);
}