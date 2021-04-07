#![allow(unused_imports)]
 
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::HashMap;

const MOD:usize = 1_000_000_007;

// 線分と線分が交差するか否か
fn judge_ientersected(
  ax:f64, ay:f64,
  bx:f64, by:f64,
  cx:f64, cy:f64,
  dx:f64, dy:f64
) -> bool {
  let ta = (cx - dx) * (ay - cy) + (cy - dy) * (cx - ax);
  let tb = (cx - dx) * (by - cy) + (cy - dy) * (cx - bx);
  let tc = (ax - bx) * (cy - ay) + (ay - by) * (ax - cx);
  let td = (ax - bx) * (dy - ay) + (ay - by) * (ax - dx);

  tc * td < 0f64 && ta * tb < 0f64
  // return tc * td <= 0f64 && ta * tb <= 0f64; // 端点を含む場合
}
 
#[fastout]
fn main() {
  input! {
    chop:[f64;4],
    n:usize,
    vals:[(f64, f64);n]
  }
  
  let mut count = 0;
  for i in 0..n {
    let from = vals[i % n];
    let to = vals[(i+1)%n]; 
    if judge_ientersected(
      chop[0], chop[1],
      chop[2], chop[3],
      from.0, from.1,
      to.0, to.1
    ) {
      count += 1;
    }
  }
  println!("{}", count / 2 + 1);
}