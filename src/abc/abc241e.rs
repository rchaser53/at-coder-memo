/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::*;

fn main() {
  input! {
    n:usize,
    mut k:usize,
    vals:[usize;n]
  }

  let mut memo = vec![];
  let mut seen = HashSet::new();
  let mut now = 0;
  let mut ti = 0;
  loop {
    if seen.contains(&now) {
      ti = now;
      break
    }
    memo.push((vals[now], now));
    seen.insert(now);
    now += vals[now];
    now %= n;
  }

  let mut que = VecDeque::new();
  while let Some((v, i)) = memo.pop() {
    que.push_front(v);
    if i == ti { break }
  }

  let mut result = 0;
  if k <= memo.len() {
    for i in 0..k {
      result += memo[i].0;
    }
    println!("{}", result);
    return
  }
  
  k -= memo.len();
  for (v, _) in memo {
    result += v;
  }

  let qn = que.len();
  let num = k / qn;
  let rnum = k % qn;
  let sum = que.iter().sum::<usize>();
  result += sum * num;
  for i in 0..rnum {
    result += que[i];
  }

  println!("{}", result);
}