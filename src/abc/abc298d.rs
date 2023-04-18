/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::*;

const MOD:usize = 998244353;
fn main() {
  input!{
    q:usize,
  }

  let limit = 10usize.pow(6);
  let mut memo = vec![0;limit];
  memo[1] = 1;
  for i in 2..limit {
    memo[i] = memo[i-1] * 10 % MOD;
  }

  let mut now = 1;
  let mut que = VecDeque::new();
  que.push_back(1);

  for _ in 0..q {
    input! {
      t: usize
    }

    if t == 1 {
      input! {
        num:usize
      }
      que.push_back(num);
      now *= 10;
      now %= MOD;
      now += num;
      now %= MOD;
    } else if t == 2 {
      let len = que.len();
      let v = que.pop_front().unwrap() * memo[len] % MOD;
      now = (MOD + now - v) % MOD;
    } else {
      println!("{}", now);
    }
  }
}