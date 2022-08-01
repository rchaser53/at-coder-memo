/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use std::collections::*;
use std::cmp::Reverse;
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;

const MOD:usize = 998244353;
fn main() {
  input! {
    n:usize,
    vals:[usize;n]
  }
  
  let mut result = 0usize;
  for num in 1..=n {
    // [個数][余り]
    let mut memo = vec![vec![0;num];num+1];
    memo[0][0] = 1;

    // j要素目
    for j in 0..n {
      let mut new_memo = memo.clone();
      let v = vals[j];
      // 余り
      for k in 0..num {
        let ni = (v + k) % num;
        // 個数
        for l in 0..num {
          new_memo[l+1][ni] += memo[l][k];
          new_memo[l+1][ni] %= MOD;
        }
      }
      memo = new_memo;
    }

    result += memo[num][0];
    result %= MOD;
  }

  println!("{}", result);
}