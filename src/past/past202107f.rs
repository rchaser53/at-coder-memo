use proconio::input;
use proconio::marker::*;
use std::collections::*;

fn main() {
  input!{
    n:usize,
    mut vals:[(usize, usize, usize);n]
  }

  vals.sort_by(|a,b|a.0.cmp(&b.0));

  let mut i = 0;
  let mut last = vals[0].0;
  while i < n {
    last = vals[i].0;
    let mut targets = vec![];
    while i < n && last == vals[i].0 {
      targets.push(vals[i]);
      i += 1;
    }

    let mut memo = vec![false;48];
    for (_, s, g) in targets {
      let rs = s * 2;
      let rg = (g * 2).saturating_sub(1);

      for i in rs..=rg {
        if memo[i] {
          println!("Yes");
          return
        }
        memo[i] = true;
      }
    }
  }
  println!("No");
}