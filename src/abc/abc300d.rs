/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::cmp::Reverse;

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
  input!{
    n:usize,
  }
  
  let primes = create_primes(10usize.pow(6));
  let mut result = 0;
  let m = primes.len();
  for ai in 0..m {
    let a = primes[ai];
    let mut temp = 0;
    for bi in ai+1..m {
      let b = primes[bi];

      let aab = a * a * b;
      let mut count = 0;
      for ci in bi+1..m {
        let c = primes[ci];
        if n < aab * c * c {
          break
        }
        count += 1;
      }
      result += count;
      if count == 0 {
        break
      }
      temp += 1;
    }
    if temp == 0 {
      break
    }
  }
  println!("{}", result);
}