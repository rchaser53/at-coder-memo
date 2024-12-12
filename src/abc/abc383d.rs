/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::marker::PhantomData;
use std::cmp::*;
use std::collections::*;

fn create_primes(n:usize) -> Vec<usize> {
  let mut memo = vec![true;n+1];
  memo[0] = false;
  memo[1] = false;
  for i in 2..=n {
    if !memo[i] { continue }
    for j in 2.. {
      let ni = i * j;
      if n < ni { break }
      memo[ni] = false;
    }
  }

  let mut primes = vec![];
  for i in 0..memo.len() {
    let v = memo[i];
    if v {
      primes.push(i);
    }
  }
  primes
}

fn main() {
  input! {
    n:usize
  }

  let arr = create_primes(2*10usize.pow(6)+10);
  let mut result = 0;
  let m = arr.len();
  for i in 0..m {
    let v = arr[i];
    if n < v.pow(8) {
      break
    }
    result += 1;
  }

  let mut que = arr.into_iter().collect::<VecDeque<usize>>();
  while let Some(v1) = que.pop_front() {
    let v1_2 = v1*v1;
    while let Some(v2) = que.pop_back() {
      let v2_2 = v2*v2;
      if v1_2 * v2_2 <= n {
        que.push_back(v2);
        result += que.len();
        break
      }
    }
  }

  println!("{}", result);
}