/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use std::collections::*;
use std::cmp::Reverse;
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;

type Target = usize;
type UseValue = usize;
fn lower_bound(arr: &Vec<Target>, x: &UseValue) -> usize {
    let mut low = 0;
    let mut high = arr.len();
    while low != high {
        let mid = (low + high) / 2;
        // NEEDS TO EDIT
        match arr[mid].cmp(x) {
            std::cmp::Ordering::Less => {
                low = mid + 1;
            }
            std::cmp::Ordering::Equal | std::cmp::Ordering::Greater => {
                high = mid;
            }
        }
    }
    low
}

const MOD:usize = 1_000_000_007;
fn main() {
  input! {
    n:usize,
    w:usize,
    l:usize,
    r:usize,
    mut xs:[usize;n]
  }
  xs.insert(0, 0);
  xs.insert(n+1, w);

  let n = n + 2;
  let mut ps = vec![(0,0);n];
  ps[0] = (0,0);
  let mut dp = vec![0;n];
  let mut now = 0;
  dp[0] = 1;
  for i in 0..n {
    now += ps[i].0;
    now += MOD;
    now -= ps[i].1;
    now %= MOD;
    dp[i] += now;
    dp[i] %= MOD;

    let cv = xs[i];
    let lti = lower_bound(&xs, &(cv+l));
    if lti <= n-1 {
      ps[lti].0 += dp[i];
      ps[lti].0 %= MOD;
    }

    let rti = lower_bound(&xs, &(cv+r));
    if rti <= n-1 {
      if xs[rti] == cv+r {
        if rti <= n-2 {
          ps[rti+1].1 += dp[i];
          ps[rti+1].1 %= MOD;
        }
      } else {
        ps[rti].1 += dp[i];
        ps[rti].1 %= MOD;
      }
    }
  }
  println!("{}", dp[n-1]);
}