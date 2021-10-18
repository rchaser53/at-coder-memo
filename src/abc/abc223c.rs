use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::Reverse;

const MOD:usize = 998244353;

fn main() {
  input!{
    n:usize,
    mut vals:[(f64,f64);n]
  }

  let mut ltime = vec![0f64;n+1];
  let mut rtime = vec![0f64;n+1];
  let mut tot = vec![0f64;n+1];
  
  for i in 1..=n {
    tot[i] = vals[i-1].0 + tot[i-1];
    ltime[i] = vals[i-1].0 / vals[i-1].1 + ltime[i-1];
  }

  for i in (0..n).rev() {
    rtime[i] = vals[i].0 / vals[i].1 + rtime[i+1];
  }

  for li in 0..=n-1 {
    let (distance, speed) = vals[li];
    if rtime[li+1] <= ltime[li] && ltime[li] <= rtime[li] {
      let diff = ltime[li] - rtime[li+1];
      let left = distance - diff * speed;
      println!("{}", tot[li] + left / 2f64);
      return
    } else if rtime[li+1] <= ltime[li+1] && ltime[li+1] <= rtime[li] {
      let diff = rtime[li+1] - ltime[li];
      let add = diff * speed;
      let left = distance - diff * speed;
      println!("{}", tot[li] + add + left / 2f64);
      return
    }
  }
}