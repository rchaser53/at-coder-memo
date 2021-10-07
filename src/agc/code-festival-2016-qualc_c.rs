use proconio::input;
use proconio::marker::*;
use std::collections::*;
 
const MOD:usize = 1_000_000_007;
pub fn main(
) {
  input! {
    n:usize,
    a:[usize;n],
    b:[usize;n]
  }

  let mut memoa = vec![(0,0);n];
  memoa[0] = (a[0], a[0]);
  for i in 1..n {
    if memoa[i-1].1 < a[i] {
      memoa[i] = (a[i], a[i]);
    } else {
      memoa[i] = (1, memoa[i-1].1);
    }
  }

  let mut memob = vec![(0,0);n];
  memob[n-1] = (b[n-1], b[n-1]);
  for i in (0..n-1).rev() {
    if memob[i+1].1 < b[i] {
      memob[i] = (b[i], b[i]);
    } else {
      memob[i] = (1, memob[i+1].1);
    }
  }

  let mut result = 1usize;
  for i in 0..n {
    let (alv, ahv) = memoa[i];
    let (blv, bhv) = memob[i];

    let lv = std::cmp::max(alv, blv);
    let hv = std::cmp::min(ahv, bhv);

    if hv < lv {
      println!("0");
      return
    }
    result *= hv - lv + 1;
    result %= MOD;
  }

  println!("{}", result);
}