#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;
use std::collections::*;

fn main() {
  input! {
    n: usize,
    kk: usize,
    vals:[(isize, isize);n]
  }

  let mut min = 4100000000000000000isize;
  for i in 0..n {
    let (x1, y1) = vals[i];
    for j in 0..n {
      let (x2, y2) = vals[j];
      for k in 0..n {
        let (x3, y3) = vals[k];
        for l in 0..n {
          let (x4, y4) = vals[l];
          
          let mut xs = vec![x1,x2,x3,x4];
          let mut ys = vec![y1,y2,y3,y4];
          xs.sort();
          ys.sort();
          let (sx, lx) = (xs[0], xs[3]);
          let (sy, ly) = (ys[0], ys[3]);
          
          let mut count = 0;
          for m in 0..n {
            let (tx, ty) = vals[m];
            if sx <= tx && tx <= lx && sy <= ty && ty <= ly {
              count += 1;
            }
          }
          
          if kk <= count {
            min = std::cmp::min((lx-sx).abs() * (ly-sy).abs(), min);
          }
        }
      }
    }
  }
  println!("{}", min);
}