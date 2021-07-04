#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;
use std::collections::VecDeque;
 
fn main() {
  input!{
    t:f64,
    l:f64,
    x:f64,
    y:f64,
    q:usize,
    vals:[f64;q]
  }
  
  let tox = 0f64;
  let toy = -l/2f64;
  let base_theta = 2f64 * std::f64::consts::PI;
 
  for v in vals {
    let theta = base_theta - base_theta * v / t;
    let y1 = tox * theta.cos() - toy * theta.sin();
    let z1 = tox * theta.sin() + toy * theta.cos() - toy;
    let rx = (x.powi(2) + (y1-y).powi(2)).sqrt();
    println!("{}", z1.atan2(rx) / std::f64::consts::PI * 180f64);
  }
}