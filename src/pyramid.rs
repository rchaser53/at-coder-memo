#![allow(unused_imports)]
 
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::HashMap;

#[fastout]
fn main() {
  input! {
    n: usize,
    mut vals: [(isize, isize, isize);n]
  }
  
  vals = vals
          .into_iter()
          .filter(|(_,_,v)| v != &0)
          .collect::<Vec<(isize, isize, isize)>>();
  
  if vals.len() == 1 {
    println!("{} {} {}", vals[0].0, vals[0].1, vals[0].2);
    return
  }
  
  let mut succeed = true;
  for x in 0..=100 {
    let xx = x as isize;
    for y in 0..=100 {
      succeed = true;
      let yy = y as isize;
      let mut last = 0;
      let mut start = false;

      for i in 0..vals.len() {
        if start {
          let v = (xx - vals[i].0).abs() + (yy - vals[i].1).abs() + vals[i].2;
          if last != v {
            succeed = false;
            break
          }
        } else {
          last = (xx - vals[i].0).abs() + (yy - vals[i].1).abs() + vals[i].2;
          start = true;
        }
      }
      
      if succeed {
        println!("{} {} {}", x, y, last);
        return
      }
    }
  }
}