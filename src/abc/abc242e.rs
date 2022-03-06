/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::*;

const MOD:usize = 998244353;
fn main() {
  input! {
    t:usize
  }

  let mut memo = vec![0;10usize.pow(6)+1];
  memo[0] = 1;
  for i in 0..10usize.pow(6) {
    memo[i+1] = memo[i] * 26;
    memo[i+1] %= MOD;
  }

  for _ in 0..t {
    input!{
      n:usize,
      s:Chars
    }

    let mut result = 0;
    let mut limit = n/2;
    if n % 2 == 1 {
      limit += 1;
    }
    for i in 0..limit {
      let v = (s[i] as u8 - 'A' as u8) as usize;
      let left = limit - i - 1;
      let v = memo[left] * v % MOD;
      result += v;
      result %= MOD;
    }

    let hi = n/2;
    let padding = if n % 2 == 0 {
      0
    } else {
      1
    };

    let mut success = true;
    for i in 0..hi {
      if s[hi-1-i] < s[hi+i+padding] {
        break
      } else if s[hi-1-i] > s[hi+i+padding] {
        success = false;
        break
      }
    }
    if success {
      result += 1;
      result %= MOD;
    }

    println!("{}", result);
  }
}