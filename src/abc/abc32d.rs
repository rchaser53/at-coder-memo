/* OUTPUT FILE */
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_assignments)]
use proconio::input;
use proconio::marker::*;
use std::collections::*;

fn case1(
  n:usize,
  w:usize,
  vals: Vec<(usize,usize)>
) {
  let mut before = vec![];
  let mut after = vec![];
  for i in 0..n/2 {
    before.push(vals[i]);
  }
  for i in n/2..n {
    after.push(vals[i]);
  }

  let bn = before.len();
  let blimit = 1 << bn;  
  let mut br = vec![];
  for i in 0..blimit {
    let mut tv = 0;
    let mut tw = 0;
    for j in 0..n {
      if i >> j & 1 == 1 {
        tv += before[j].0;
        tw += before[j].1;
      }
    }
    if tw <= w {
      br.push((tv, tw));
    }
  }

  br.sort_by(|a, b| a.1.cmp(&b.1));
  for i in 1..br.len() {
    br[i].0 = std::cmp::max(br[i].0, br[i-1].0);
  }
  
  let mut result = 0;
  let an = after.len();
  let alimit = 1 << an;
  for i in 0..alimit {
    let mut tv = 0;
    let mut tw = 0;
    for j in 0..n {
      if i >> j & 1 == 1 {
        tv += after[j].0;
        tw += after[j].1;
      }
    }
    
    if w < tw { continue }
    let left = w - tw;

    let ti = match br.binary_search_by(|a| a.1.cmp(&left)) {
      Ok(ti) => ti,
      Err(ti) => ti - 1
    };

    result = std::cmp::max(result, tv + br[ti].0);
  }
  println!("{}", result);
}

// wait case
fn case2(
  n:usize,
  w:usize,
  max_w:usize,
  vals: Vec<(usize,usize)>
) {
  let limit = std::cmp::min(n * max_w, w);
  let mut dp = vec![0;limit+1];

  for i in 0..n {
    let (cv, cw) = vals[i];
    let mut new = dp.clone();
    for j in 0..=limit {
      if j + cw <= limit {
        new[j+cw] = std::cmp::max(dp[j]+cv, new[j+cw]);
      }
    }
    dp = new;
  }
  let mut result = 0;
  for i in 0..=limit {
    result = std::cmp::max(result, dp[i]);
  }
  println!("{}", result);
}

// value case
fn case3(
  n:usize,
  w:usize,
  max_value:usize,
  vals: Vec<(usize,usize)>
) {
  let default_value = 1_000_000_000_000usize;
  let limit = n * max_value;
  let mut dp = vec![default_value;limit+1];
  dp[0] = 0;

  for i in 0..n {
    let (cv, cw) = vals[i];
    let mut new = dp.clone();
    for j in 0..=limit {
      let ww = dp[j];
      if ww + cw <= w && cv + j <= limit {
        new[cv+j] = std::cmp::min(ww+cw, new[cv+j]);
      }
    }
    dp = new;
  }

  let mut result = 0;
  for i in 0..=limit {
    if dp[i] != default_value {
      result = i;
    }
  }
  println!("{}", result);
}

pub fn main(
) {
  input! {
    n:usize,
    w:usize,
    vals:[(usize,usize);n]
  }

  if n <= 30 {
    case1(n,w,vals);
    return
  }

  let mut maxw = 0;
  let mut maxv = 0;
  for i in 0..n {
    maxv = std::cmp::max(maxv, vals[i].0);
    maxw = std::cmp::max(maxw, vals[i].1);
  }

  if maxw <= 1000 {
    case2(n,w,maxw,vals);
  } else {
    case3(n,w,maxv,vals);
  }
}
