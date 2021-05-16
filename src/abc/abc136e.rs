use proconio::input;
use std::collections::*;
 
fn main() {
  input! {
    n:usize,
    k:usize,
    mut vals:[usize;n]
  }
  vals.sort();
  let total = vals.iter().sum::<usize>();
  let mut set = HashSet::new();
  let mut i = 1;
  while i * i <= total {
    if total % i == 0 {
      set.insert(i);
      set.insert(total/i);
    }
    i += 1;
  }
  set.remove(&1);

  let mut max = 1;
  for tv in set {
    let mut temps = vals
      .iter()
      .map(|v| v % tv)
      .collect::<Vec<usize>>();
    temps.sort();
    
    let mut c1 = 0;
    let mut c2 = 0;
    let mut r = n - 1;
    for l in 0..n {
      c1 += temps[l];
      while c2 < c1 {
        c2 += tv - temps[r];
        r -= 1;
      }
      if r <= l {
        break;
      }
    }
    
    if c1 <= k {
      max = std::cmp::max(max, tv);
    }
  }
  
  println!("{}", max);
}